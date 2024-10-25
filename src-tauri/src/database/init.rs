use rusqlite::{self, Connection, Result};

pub fn init(conn: &Connection) -> Result<()> {
    init_account(conn)?;
    init_transaction(conn)?;
    init_details(conn)?;
    Ok(())
}

fn init_account(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE ACCOUNT(
        name VARCHAR(255) NOT NULL PRIMARY KEY ,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_details(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE DETAIL(
        id TEXT PRIMARY KEY,
        trans_id TEXT NOT NULL,
        account TEXT NOT NULL,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_transaction(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE TRANS(
        id TEXT PRIMARY KEY,
        time DATE NOT NULL,
        extra VARCHAR(255)
    )
    ",
        (),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};
    use rust_decimal::Decimal;

    use crate::database::{
        account::{self, Account},
        details::Details,
        transaction::Transaction,
    };

    use super::*;
    #[test]
    fn test_account() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_account(&conn)?;
        conn.execute(
            "INSERT INTO ACCOUNT (name,currency,balance) VALUES (?1,?2,?3)",
            ( "a", "b", 0.1),
        )?;
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
    fn test_details() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_details(&conn)?;
        conn.execute(
            "INSERT INTO DETAIL (id,trans_id,account,currency,balance) VALUES (?1,?2,?3,?4,?5)",
            (0, 1, 2, "CNY", 0.1),
        )?;
        let mut stmt =
            conn.prepare("SELECT id,trans_id,account,currency,balance FROM DETAIL")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Details {
                id: row.get(0)?,
                trans_id: row.get(1)?,
                account: row.get(2)?,
                currency: row.get(3)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }

    #[test]
    fn test_trans() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_transaction(&conn)?;
        conn.execute(
            "INSERT INTO TRANS (id,time,extra) VALUES (?1,?2,?3)",
            (0, "2023-08-21", "CNY"),
        )?;
        let mut stmt = conn.prepare("SELECT id,extra FROM TRANS")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                date: row.get(1)?,
                extra: row.get(2)?,
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }
}
