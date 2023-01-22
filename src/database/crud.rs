use super::models::{Customer, TransferHuman};
use crate::database::models;
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use std::path::Path;

static DATABASE_FILE: &str = "mydb.sqlite";
enum Table {
    CUSTOMER,
    TRANSFER,
}
impl Table {
    fn as_str(&self) -> &str {
        match self {
            Table::CUSTOMER => "customers",
            Table::TRANSFER => "transfers",
        }
    }
}

fn get_connection() -> Result<Connection> {
    Connection::open(DATABASE_FILE)
}

pub fn create_db() -> Result<()> {
    get_connection().unwrap().execute_batch(
        "BEGIN;
            CREATE TABLE IF NOT EXISTS customers (id INTEGER PRIMARY KEY, name TEXT NOT NULL, balance INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS transfers (id INTEGER PRIMARY KEY, created_at TEXT NOT NULL, from_id INTEGER NOT NULL, to_id INTEGER NOT NULL, amount REAL NOT NULL);
            COMMIT;",
    )?;
    Ok(())
}

pub fn check_db() -> Result<()> {
    if !Path::new(DATABASE_FILE).exists() {
        create_db()?;
    }
    Ok(())
}

pub fn create_customer(customer: &models::Customer) -> Result<()> {
    let mut starting_balance: f64 = 0.0;

    if customer.balance.is_some() {
        starting_balance = customer.balance.unwrap();
    }

    get_connection().unwrap().execute(
        "INSERT INTO customers (name, balance, created_at) VALUES (?1, ?2, ?3)",
        params![customer.name, starting_balance, customer.created_at],
    )?;
    Ok(())
}

pub fn get_customer(id: u16) -> Result<Customer> {
    let conn = get_connection().unwrap();
    let query = format!("SELECT * FROM {} WHERE id = ?", Table::CUSTOMER.as_str());
    let mut stmt = conn.prepare(&query)?;

    let user = stmt.query_row(params![id], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;
    Ok(user)
}

pub fn update_balance(id: u16, balance: f64) -> Result<()> {
    let conn = get_connection().unwrap();

    let query = format!(
        "UPDATE {} SET balance = ?1 WHERE id = ?2",
        Table::CUSTOMER.as_str()
    );

    conn.execute(&query, params![balance, id]).unwrap();
    Ok(())
}

pub fn create_transfer(id_from: u16, id_to: u16, amount: f64) -> Result<()> {
    let created_at = Utc::now().to_rfc2822();

    let query = format!(
        "INSERT INTO {} (created_at, from_id, to_id, amount) VALUES (?1, ?2, ?3, ?4)",
        Table::TRANSFER.as_str()
    );

    get_connection()
        .unwrap()
        .execute(&query, params![created_at, id_from, id_to, amount])?;
    Ok(())
}

pub fn edit_customer(id: u16, customer: models::CustomerEdit) -> Result<()> {
    let conn = get_connection().unwrap();

    let query = format!(
        "UPDATE {} SET name = ?1 WHERE id = ?2",
        Table::CUSTOMER.as_str()
    );

    conn.execute(&query, params![customer.name, id]).unwrap();
    Ok(())
}

pub fn get_all_customers() -> Result<Vec<Customer>> {
    let mut record_list: Vec<Customer> = Vec::new();

    let conn = get_connection().unwrap();
    let query = format!("SELECT * FROM {}", Table::CUSTOMER.as_str());
    let mut stmt = conn.prepare(&query)?;

    stmt.query_map(params![], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?
    .for_each(|i| record_list.push(i.unwrap()));

    Ok(record_list)
}

pub fn get_all_transfers() -> Result<Vec<TransferHuman>> {
    let mut record_list: Vec<TransferHuman> = Vec::new();

    let conn = get_connection().unwrap();
    let query = format!(
        "SELECT {}.id, {}.created_at, from_c.name, to_c.name, {}.amount
    FROM {}
    JOIN {} as from_c ON {}.from_id = from_c.id
    JOIN {} as to_c ON {}.to_id = to_c.id;
    ",
        Table::TRANSFER.as_str(),
        Table::TRANSFER.as_str(),
        Table::TRANSFER.as_str(),
        Table::TRANSFER.as_str(),
        Table::CUSTOMER.as_str(),
        Table::TRANSFER.as_str(),
        Table::CUSTOMER.as_str(),
        Table::TRANSFER.as_str(),
    );

    let mut stmt = conn.prepare(&query)?;

    stmt.query_map(params![], |row| {
        Ok(TransferHuman {
            id: row.get(0)?,
            created_at: row.get(1)?,
            name_from: row.get(2)?,
            name_to: row.get(3)?,
            amount: row.get(4)?,
        })
    })?
    .for_each(|i| record_list.push(i.unwrap()));

    Ok(record_list)
}
