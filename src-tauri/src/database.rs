use chrono::Datelike;
use details::Details;
use rusqlite::params;
use rusqlite::Connection;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
pub mod account;
pub mod details;
use error::Error;

use crate::error;
pub mod init;
pub mod transaction;

/// Calculates the total income for the current month.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * The total income as a `Decimal`.
pub fn get_month_incomed(conn: &Connection) -> Result<Decimal, Error> {
  let mut result = Decimal::zero();
    // Prepare and execute SQL statement to select relevant transactions for the current month.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    // Iterate over the results to calculate total income.
    let iter = stmt.query_map(params![], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(3)?).unwrap(),
        })
    })?;
    for i in iter {
        let details = i.unwrap();
        // If the transaction is related to income, add its balance to the total.
        if details.account.starts_with("income::") {
            result += details.balance;
        }
    }

    Ok(result)
}

/// Calculates the total expenses for the current month.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * The total expenses as a `Decimal`.
pub fn get_month_expenses(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    // Prepare and execute SQL statement to select relevant transactions for the current month.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    // Iterate over the results to calculate total expenses.
    let iter = stmt.query_map(params![], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(3)?).unwrap(),
        })
    })?;
    for i in iter {
        let details = i.unwrap();
        // If the transaction is related to expenses, add its balance to the total.
        if details.account.starts_with("expenses::") {
            result += details.balance;
        }
    }

    Ok(result)
}

/// Recalculates account balances and updates them in the database.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * An `Result<()>` indicating success or failure of the operation.
pub fn recalcute(conn: &Connection) -> Result<(), Error> {
    // Get all accounts from the database.
    let accounts = account::get_accounts(conn)?;
    for account in accounts {
        // Get details for each account and calculate the total balance.
        let details = details::get_details_by_account(conn, account.name.as_str())?;
        let mut balance = dec!(0.0);
        for i in details {
            balance += i.balance;
        }
        // Update the account's balance with the calculated value.
        let mut acc = account;
        acc.balance = balance;
        account::update_account(conn, &acc)?;
    }
    Ok(())
}

/// Calculates weekly expenses for the current week.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * A vector of f32 representing daily expenses from Monday to Sunday.
pub fn get_weekly_expenses(conn: &Connection) -> Result<Vec<f32>, Error> {
    let mut result = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Prepare and execute SQL statement to select relevant transactions for the current week.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,balance,time FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y %W', time) = strftime('%Y %W', 'now') ORDER BY TIME")?;
    // Iterate over the results to calculate weekly expenses.
    let iter = stmt.query_map(params![], |row: &rusqlite::Row<'_>| -> rusqlite::Result<(Details, String)> {
        Ok((Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(3)?).unwrap(),
        }, row.get(4)?))
    })?;
    for i in iter {
        let s = i?;
        // Skip income transactions.
        if s.0.account.starts_with("income") {
            continue;
        }
        // Parse the date and add the balance to the corresponding day of the week.
        let date = chrono::NaiveDateTime::parse_from_str(s.1.as_str(), "%Y-%m-%d %H:%M").unwrap();
        result[date.weekday().num_days_from_monday() as usize] += s.0.balance.to_f32().unwrap();
    }

    Ok(result)
}

/// Calculates weekly income for the current week.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * A vector of f32 representing daily income from Monday to Sunday.
pub fn get_weekly_income(conn: &Connection) -> Result<Vec<f32>, Error> {
    let mut result = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Prepare and execute SQL statement to select relevant transactions for the current week.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,balance,time FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y %W', time) = strftime('%Y %W', 'now') ORDER BY TIME")?;
    // Iterate over the results to calculate weekly income.
    let iter = stmt.query_map(params![], |row: &rusqlite::Row<'_>| -> rusqlite::Result<(Details, String)> {
        Ok((Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(3)?).unwrap(),
        }, row.get(4)?))
    })?;
    for i in iter {
        let s = i?;
        // Skip expenses transactions.
        if s.0.account.starts_with("expenses") {
            continue;
        }
        // Parse the date and add the balance to the corresponding day of the week.
        let date = chrono::NaiveDateTime::parse_from_str(s.1.as_str(), "%Y-%m-%d %H:%M").unwrap();
        result[date.weekday().num_days_from_monday() as usize] += s.0.balance.to_f32().unwrap();
    }

    Ok(result)
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{account, details, init, transaction};
    use crate::database::recalcute;
    use error::Error;
    use rust_decimal_macros::dec;

    #[test]
    fn test_recalcute() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        account::add_account(&conn, "income::bar", "USD","","")?;
        account::add_account(&conn, "expenses::foo", "USD","","")?;

        let date = chrono::Utc::now();
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            Decimal::from_f32_retain(-10.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
            Decimal::from_f32_retain(10.0).unwrap(),
        )?;
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            Decimal::from_f32_retain(-12.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
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