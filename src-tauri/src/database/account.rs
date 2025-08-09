use rusqlite::{params, Connection, Result};
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub balance: Decimal,
    pub currency: String,
    pub icon: Option<String>,
    pub extra: Option<String>,
}

// Function to add a new account to the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
/// * `name` - The name of the account as a string slice.
/// * `currency` - The currency of the account as a string slice.
/// * `icon` - An optional icon for the account as a string slice.
/// * `extra` - Any additional information about the account as a string slice.
///
/// # Returns
/// * `Result<()>` - On success, returns Ok(()). On failure, returns an error.
pub fn add_account(
    conn: &Connection,
    name: &str,
    currency: &str,
    icon: &str,
    extra: &str,
) -> Result<()> {
    // SQLite query to insert a new account into the database
    conn.execute(
        "INSERT INTO ACCOUNT (name,currency,balance,icon,extra ) VALUES (?1,?2,?3,?4,?5)",
        params![name, currency, 0.0, icon, extra],
    )?;
    Ok(())
}
/// Function to retrieve account information from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
/// * `name` - The name of the account as a string slice.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn read_account(conn: &Connection, name: &str) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select account information by name
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name = ?")?;

    // Map each row from the result set into an Account struct
    let iter = stmt.query_map(params![name], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;

    // Collect the results into a vector
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}
/// Function to remove an account from the database by name
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
/// * `name` - The name of the account as a string slice.
///
/// # Returns
/// * `Result<()>` - On success, returns Ok(()). On failure, returns an error.
pub fn del_account(conn: &Connection, name: &str) -> Result<()> {
    // SQLite query to delete an account from the database by name
    conn.execute("DELETE FROM ACCOUNT WHERE name=?1", params![name])?;
    Ok(())
}

/// Function to get all accounts from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select all accounts from the database
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Function to get all income accounts from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn get_income_accounts(conn: &Connection) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select all accounts with names starting with 'income::' from the database
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name LIKE 'income::%'")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Function to get all expense accounts from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn get_expenses_accounts(conn: &Connection) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select all accounts with names starting with 'expenses::' from the database
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name LIKE 'expenses::%'")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Function to get all asset accounts from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn get_assets_accounts(conn: &Connection) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select all accounts with names starting with 'assets::' from the database
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name LIKE 'assets::%'")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Function to get all liability accounts from the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
///
/// # Returns
/// * `Result<Vec<Account>>` - On success, returns a vector of Account structs. On failure, returns an error.
pub fn get_liabilities_accounts(conn: &Connection) -> Result<Vec<Account>> {
    // Prepare and execute the SQL query to select all accounts with names starting with 'liabilities::' from the database
    let mut stmt = conn.prepare("SELECT * FROM ACCOUNT WHERE name LIKE 'liabilities::%'")?;
    let iter = stmt.query_map([], |row| {
        Ok(Account {
            name: row.get(0)?,
            currency: row.get(1)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
            icon: row.get(3)?,
            extra: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?)
    }
    Ok(result)
}

/// Function to update an account in the database
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
/// * `account` - An Account struct containing updated information.
///
/// # Returns
/// * `Result<()>` - On success, returns Ok(()). On failure, returns an error.
pub fn update_account(conn: &Connection, account: &Account) -> Result<()> {
    // SQLite query to update the specified account in the database with new values
    conn.execute(
        "UPDATE ACCOUNT SET currency=?2,balance=?3,icon=?4,extra=?5 WHERE name=?1",
        (
            account.name.clone(),
            account.currency.clone(),
            account.balance.to_f32(),
            account.icon.clone(),
            account.extra.clone(),
        ),
    )?;
    Ok(())
}

/// Function to update an account's balance by adding a delta amount
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection object.
/// * `account_name` - The name of the account to update.
/// * `delta` - The amount to add to the current balance (can be negative).
///
/// # Returns
/// * `Result<()>` - On success, returns Ok(()). On failure, returns an error.
pub fn update_account_balance(conn: &Connection, account_name: &str, delta: Decimal) -> Result<()> {
    // First, get the current account information
    let mut accounts = read_account(conn, account_name)?;
    if accounts.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows.into());
    }
    
    // Update the balance by adding the delta
    let mut account = accounts.remove(0);
    account.balance += delta;
    
    // Update the account in the database
    update_account(conn, &account)?;
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
        add_account(&conn, "someone", "CNY", "", "")?;

        let mut stmt = conn.prepare("SELECT name,currency,balance,icon,extra FROM ACCOUNT")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Account {
                name: row.get(0)?,
                currency: row.get(1)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
                icon: row.get(3)?,
                extra: row.get(4)?,
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
        add_account(&conn, "a", "b", "", "")?;
        let accounts = get_accounts(&conn)?;
        let a = read_account(&conn, &accounts[0].name)?;
        println!("{:?}", a);

        Ok(())
    }
    #[test]
    fn test_get_last_id() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b", "", "")?;

        let accounts = get_accounts(&conn)?;

        assert_eq!(1, accounts.len());
        Ok(())
    }
    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init(&conn)?;
        add_account(&conn, "a", "b", "", "")?;
        let id = get_accounts(&conn)?;
        del_account(&conn, &id[0].name)?;

        assert_eq!(0, get_accounts(&conn)?.len());
        Ok(())
    }
}
