use rusqlite::{params, Connection, Result};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub balance: Decimal,
    pub currency: String,
}

pub fn add_account(conn: &Connection, name: &str, currency: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO ACCOUNT (name,currency,balance) VALUES (?1,?2,?3)",
        (name, currency, 0.0),
    )?;
    Ok(())
}

/// Get info of account
pub fn read_account(conn: &Connection, name: &str) -> Result<Vec<Account>> {
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name = ?")?;
    let iter = stmt.query_map(params![name], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Remove account
pub fn del_account(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM ACCOUNT WHERE name=?1", params![name])?;
    Ok(())
}

/// Get all account
pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>> {
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

pub fn update_account(conn: &Connection, account: &Account) -> Result<()> {
    conn.execute(
        "UPDATE ACCOUNT SET currency=?2,balance=?3 WHERE name=?1",
        (
            account.name.clone(),
            account.currency.clone(),
            account.balance.to_f32(),
        ),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::init::init;

    use super::*;
    #[test]
    fn test_add_account() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "someone", "CNY")?;

        let mut stmt = conn.prepare("SELECT name,currency,balance FROM ACCOUNT")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Account {
                name: row.get(0)?,
                currency: row.get(1)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            })
        })?;
        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }
    #[test]
    fn test_read_account() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        let accounts = get_accounts(&conn)?;
        let a = read_account(&conn, &accounts[0].name)?;
        println!("{:?}", a);

        Ok(())
    }
    #[test]
    fn test_get_last_id() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;

        let accounts = get_accounts(&conn)?;

        assert_eq!(1, accounts.len());
        Ok(())
    }
    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b")?;
        let id = get_accounts(&conn)?;
        del_account(&conn, &id[0].name)?;

        assert_eq!(0, get_accounts(&conn)?.len());
        Ok(())
    }
}
