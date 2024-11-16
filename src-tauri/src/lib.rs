use std::{collections::HashMap, fs, sync::Mutex};

use database::{
    account::{self, Account},
    get_month_expenses, get_month_incomed, get_weekly_expenses, get_weekly_income, init,
};
use rusqlite::Connection;
use rust_decimal::prelude::*;
use serde_json::{json, Map};
use tauri_struct::{AccountIconName, WeeklyIncomeExpenses};
mod database;
mod error;
mod tauri_struct;
struct ConnectionWrapper(pub Mutex<Connection>);

#[tauri::command]
fn get_income_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.0.lock().unwrap();
    let accounts = account::get_income_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
        
        }).collect();
    return accounts;
}

#[tauri::command]
fn get_expenses_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.0.lock().unwrap();
    let accounts = account::get_expenses_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
        })
        .collect();
    return accounts;
}

#[tauri::command]
fn get_income_expenses(conn: tauri::State<ConnectionWrapper>) -> Vec<f32> {
    let conn = conn.0.lock().unwrap();
    // let bar_id = account::add_account(&conn, "income::a", "USD").unwrap();
    // let foo_id = account::add_account(&conn, "expenses::a", "USD").unwrap();

    // let date = chrono::Utc::now();
    // let id = transaction::add_transaction(&conn, date, "").unwrap();
    // details::add_details(
    //     &conn,
    //     id.as_str(),
    //     "income::a",
    //     "USD",
    //     Decimal::from_f32_retain(12.0).unwrap(),
    // );
    // details::add_details(
    //     &conn,
    //     id.as_str(),
    //     "expenses::a",
    //     "USD",
    //     Decimal::from_f32_retain(-12.0).unwrap(),
    // );
    // recalcute(&conn).unwrap();
    let result = vec![
        get_month_incomed(&conn).unwrap().to_f32().unwrap() * -1.0,
        get_month_expenses(&conn).unwrap().to_f32().unwrap(),
    ];
    // print!("{}", result);
    result
}

#[tauri::command]
fn get_weekly_income_expenses(conn: tauri::State<ConnectionWrapper>) -> WeeklyIncomeExpenses {
    let conn = conn.0.lock().unwrap();
    let incomes = get_weekly_income(&conn).unwrap();
    let expenses = get_weekly_expenses(&conn).unwrap();
    return WeeklyIncomeExpenses {
        income: incomes,
        expenses: expenses,
    };
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = fs::create_dir(
        dirs::home_dir()
            .unwrap()
            .join("Money-bill")
            .to_str()
            .unwrap(),
    );
    let conn = Connection::open(
        dirs::home_dir()
            .unwrap()
            .join("Money-bill/db.db3")
            .to_str()
            .unwrap(),
    )
    .unwrap();
    let _ = init::init(&conn);
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(ConnectionWrapper(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_income_expenses,
            get_weekly_income_expenses,
            get_expenses_accounts,
            get_income_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
