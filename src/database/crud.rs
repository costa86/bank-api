use crate::database::models;
use rusqlite::{params, Connection, Result};

use super::models::Customer;

fn get_connection() -> Result<Connection> {
    Connection::open("mydb.sqlite")
}

pub fn create_db() -> Result<()> {
    get_connection().unwrap().execute_batch(
        "BEGIN;
                        CREATE TABLE customers (id INTEGER PRIMARY KEY, name TEXT, balance INTEGER);
                        COMMIT;",
    )?;
    Ok(())
}
pub fn create_customer(customer: &models::Customer) -> Result<()> {
    get_connection().unwrap().execute(
        "INSERT INTO customers (name, balance) VALUES (?1, ?2)",
        params![customer.name, customer.balance],
    )?;
    Ok(())
}

pub fn get_customer(id: u16) -> Result<Customer> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM customers WHERE id = ?")?;
    let user = stmt.query_row(params![id], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
        })
    })?;
    Ok(user)
}

pub fn get_all_customers() -> Result<Vec<Customer>> {
    let mut record_list: Vec<Customer> = Vec::new();

    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM customers")?;
    let records = stmt.query_map([], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
        })
    })?;

    for i in records {
        record_list.push(i?);
    }

    Ok(record_list)
}
