use std::{fs, sync::Mutex};

use database::{
    account::{self},
    details::{self, Details},
    init,
    transaction::Transaction,
};
use error::Error;
use rusqlite::{params, Connection};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
mod database;
mod error;

struct ConnectionWrapper(pub Mutex<Connection>);

fn get_incomed(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    let mut stmt = conn
        .prepare("SELECT * FROM TRANS WHERE strftime('%Y-%m', time) = strftime('%Y-%m', 'now');")?;
    let iter = stmt.query_map(params![], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            extra: row.get(2)?,
        })
    })?;
    for i in iter {
        let mut stmt = conn.prepare("SELECT * FROM DETAIL WHERE trans_id=?1")?;
        let iter = stmt.query_map(params![i.unwrap().id], |row| {
            Ok(Details {
                id: row.get(0)?,
                trans_id: row.get(1)?,
                account: row.get(2)?,
                currency: row.get(3)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
            })
        })?;
        for i in iter {
            let details = i.unwrap();
            if details.account.starts_with("income::"){
                result += details.balance;
            }
        }
    }

    Ok(result)
}

fn get_expenses(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    let mut stmt = conn
        .prepare("SELECT * FROM TRANS WHERE strftime('%Y-%m', time) = strftime('%Y-%m', 'now');")?;
    let iter = stmt.query_map(params![], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            extra: row.get(2)?,
        })
    })?;
    for i in iter {
        let mut stmt = conn.prepare("SELECT * FROM DETAIL WHERE trans_id=?1")?;
        let iter = stmt.query_map(params![i.unwrap().id], |row| {
            Ok(Details {
                id: row.get(0)?,
                trans_id: row.get(1)?,
                account: row.get(2)?,
                currency: row.get(3)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
            })
        })?;
        for i in iter {
            let details = i.unwrap();
            if details.account.starts_with("expenses::"){
                result += details.balance;
            }
        }
    }

    Ok(result)
}

fn recalcute(conn: &Connection) -> Result<(), Error> {
    let accounts = account::get_accounts(conn)?;
    for account in accounts {
        let details = details::get_details_by_account(conn, account.name.as_str())?;
        let mut balance = dec!(0.0);
        for i in details {
            balance += i.balance;
        }
        let mut acc = account;
        acc.balance = balance;
        account::update_account(conn, &acc)?;
    }
    Ok(())
}

// fn verification(conn: &Connection) -> Result<(), Error> {
//     let trans = transaction::get_transactions(conn)?;
//     for tran in trans {
//         let details = details::get_details_by_trans(conn, tran.id.as_str())?;
//         let mut balance = dec!(0.0);
//         for i in details {
//             balance += i.balance;
//         }
//         if balance != Decimal::zero() {
//             return Err(Error::DetailSumNotZero(tran.id, balance.to_f32().unwrap()));
//         }
//     }
//     Ok(())
// }

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
    //     bar_id.as_str(),
    //     "USD",
    //     Decimal::from_f32_retain(12.0).unwrap(),
    // );
    // details::add_details(
    //     &conn,
    //     id.as_str(),
    //     foo_id.as_str(),
    //     "USD",
    //     Decimal::from_f32_retain(-12.0).unwrap(),
    // );
    recalcute(&conn).unwrap();
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
