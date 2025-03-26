use std::{fs, sync::Mutex};

use chrono::DateTime;
use database::{
    account::{self},
    details::add_details,
    get_month_expenses, get_month_incomed, get_weekly_expenses, get_weekly_income, init,
    transaction::add_transaction,
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
/// 添加账单信息到数据库
///
/// 该函数接收一个数据库连接状态、账户金额列表、日期和额外信息，并将这些信息添加到数据库中。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
/// - `account_amounts`: 账户金额列表，每个元素包含账户名称和金额。
/// - `date`: 交易日期，以时间戳形式表示。
/// - `extra`: 额外信息，如交易备注。
/// - `currency`: 货币类型，如 "USD"。
///
/// # 返回值
/// 如果操作成功，返回 `Ok(())`；如果出现错误，返回包含错误信息的 `Err(String)`。
async fn add_bills(
    conn: tauri::State<'_, ConnectionWrapper>,
    account_amounts: Vec<AccountAmount>,
    date: i64,
    extra: String,
    currency: String,
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    let utc_datetime: DateTime<chrono::Utc> = DateTime::from_timestamp(date, 0).unwrap();
    let trans_id = add_transaction(&conn, utc_datetime, &extra).unwrap();
    for account_amount in account_amounts {
        let amount = Decimal::from_f32_retain(account_amount.amount).unwrap();
        let _ = add_details(&conn, &trans_id, &account_amount.account, &currency, amount);
    }
    println!("{}", date);
    Ok(())
}
#[tauri::command]
/// 获取收入账户列表
///
/// 该函数从数据库中获取所有收入账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_income_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_income_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
        })
        .collect();
    return accounts;
}

#[tauri::command]
/// 获取支出账户列表
///
/// 该函数从数据库中获取所有支出账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_expenses_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_expenses_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
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
/// 获取每周的收入和支出数据
///
/// 该函数从数据库中查询当前周的收支情况，并返回一个包含收入和支出总额的结构体
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理
///
/// # 返回值
/// 返回 `WeeklyIncomeExpenses` 结构体，包含两个字段：
/// - `income`: 本周总收入
/// - `expenses`: 本周总支出
fn get_weekly_income_expenses(conn: tauri::State<ConnectionWrapper>) -> WeeklyIncomeExpenses {
    // 获取数据库连接
    let conn = conn.db.lock().unwrap();

    // 查询本周收入总额
    let incomes = get_weekly_income(&conn).unwrap();

    // 查询本周支出总额
    let expenses = get_weekly_expenses(&conn).unwrap();

    // 返回包含收支数据的结构体
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
