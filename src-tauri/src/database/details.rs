use rusqlite::{params, Connection, Result};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct Details {
    pub id: String,
    pub trans_id: String,
    pub account: String,
    pub currency: String,
    pub balance: Decimal,
}

pub fn add_details(
    conn: &Connection,
    trans_id: &str,
    account: &str,
    currency: &str,
    balance: Decimal,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO DETAIL (id,trans_id,account,currency,balance) values (?1,?2,?3,?4,?5)",
        params![id, trans_id, account, currency, balance.to_f32()],
    )?;

    Ok(())
}

pub fn get_details_by_trans(conn: &Connection, trans_id: &str) -> Result<Vec<Details>> {
    let mut stmt = conn
        .prepare("SELECT id,trans_id,account,currency,balance FROM DETAIL WHERE trans_id=?1")?;
    let iter = stmt.query_map(params![trans_id], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            currency: row.get(3)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
        })
    })?;

    let mut result = Vec::new();
    for i in iter {
        result.push(i?);
    }

    Ok(result)
}

pub fn get_details_by_account(conn: &Connection, account: &str) -> Result<Vec<Details>> {
    let mut stmt = conn.prepare(
        "SELECT id,trans_id,account,currency,balance FROM DETAIL WHERE account=?1",
    )?;
    let iter = stmt.query_map(params![account], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            currency: row.get(3)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
        })
    })?;

    let mut result = Vec::new();
    for i in iter {
        result.push(i?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {}
