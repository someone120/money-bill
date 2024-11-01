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

pub fn get_incomed(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    let iter = stmt.query_map(params![], |row| {
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
        if details.account.starts_with("income::") {
            result += details.balance;
        }
    }

    Ok(result)
}

pub fn get_expenses(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    let iter = stmt.query_map(params![], |row| {
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
        if details.account.starts_with("expenses::") {
            result += details.balance;
        }
    }

    Ok(result)
}

pub fn recalcute(conn: &Connection) -> Result<(), Error> {
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
