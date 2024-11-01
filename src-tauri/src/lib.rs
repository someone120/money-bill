use std::{fs, sync::Mutex};

use database::{
    account::{self}, details::{self, Details}, get_expenses, get_incomed, init, transaction::{self, Transaction}
};
use error::Error;
use rusqlite::{params, Connection};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
mod database;
mod error;

struct ConnectionWrapper(pub Mutex<Connection>);


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
        get_incomed(&conn).unwrap().to_f32().unwrap() * -1.0,
        get_expenses(&conn).unwrap().to_f32().unwrap(),
    ];
    // print!("{}", result);
    result
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
        .invoke_handler(tauri::generate_handler![greet, get_income_expenses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use database::recalcute;

    use crate::database::{init, transaction};

    use super::*;

    #[test]
    fn test_recalcute() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        account::add_account(&conn, "income::bar", "USD")?;
        account::add_account(&conn, "expenses::foo", "USD")?;

        let date = chrono::Utc::now();
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            "USD",
            Decimal::from_f32_retain(-10.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
            "USD",
            Decimal::from_f32_retain(10.0).unwrap(),
        )?;
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            "USD",
            Decimal::from_f32_retain(-12.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
            "USD",
            Decimal::from_f32_retain(12.0).unwrap(),
        )?;
        recalcute(&conn)?;

        let accounts = account::get_accounts(&conn)?;

        for account in accounts {
            if account.name == "expenses::foo" {
                let balance = dec!(22);
                assert_eq!(account.balance, balance);
            }
            if account.name == "income::bar" {
                let balance = dec!(-22);
                assert_eq!(account.balance, balance);
            }
        }
        Ok(())
    }
}
