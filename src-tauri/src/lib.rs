use std::{fs, sync::Mutex};

use chrono::DateTime;
use database::{
    account::{self}, details::add_details, get_month_expenses, get_month_incomed, get_weekly_expenses, get_weekly_income, init, transaction::add_transaction
};
use rusqlite::Connection;
use rust_decimal::prelude::*;
use tauri_struct::{AccountAmount, AccountIconName, WeeklyIncomeExpenses};
mod database;
mod error;
mod tauri_struct;
struct ConnectionWrapper {
    db: Mutex<Connection>,
}

#[tauri::command]
async fn add_bills(
    conn: tauri::State<'_, ConnectionWrapper>,
    account_amounts: Vec<AccountAmount>,
    date: i64,
    extra: String,
    currency: String
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    let utc_datetime: DateTime<chrono::Utc> = DateTime::from_timestamp(date, 0).unwrap();
    let trans_id = add_transaction(&conn,utc_datetime, &extra).unwrap();
    for account_amount in account_amounts {
        let amount = Decimal::from_f32_retain(account_amount.amount).unwrap();
        let _ = add_details(&conn, &trans_id, &account_amount.account,&currency, amount);
    }
    println!("{}",date);
    Ok(())
}

#[tauri::command]
fn get_income_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_income_accounts(&conn).unwrap();
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
fn get_expenses_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
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
    let conn = conn.db.lock().unwrap();
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
    let conn = conn.db.lock().unwrap();
    let incomes = get_weekly_income(&conn).unwrap();
    let expenses = get_weekly_expenses(&conn).unwrap();
    return WeeklyIncomeExpenses {
        income: incomes,
        expenses,
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
        .manage(ConnectionWrapper {
            db: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_income_expenses,
            get_weekly_income_expenses,
            get_expenses_accounts,
            get_income_accounts,
            add_bills
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
