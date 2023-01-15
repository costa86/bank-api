use crate::database::models;
use rusqlite::{params, Connection, Result};

use super::models::Customer;

fn get_connection() -> Result<Connection> {
    Connection::open("mydb.sqlite")
}

pub fn create_db() -> Result<()> {
    get_connection().unwrap().execute_batch(
        "BEGIN;
                        CREATE TABLE customers (id INTEGER PRIMARY KEY, name TEXT NOT NULL, balance INTEGER NOT NULL DEFAULT 0);
                        COMMIT;",
    )?;
    Ok(())
}
pub fn create_customer(customer: &models::Customer) -> Result<()> {
    let mut starting_balance: f64 = 0.0;

    if customer.balance.is_some() {
        starting_balance = customer.balance.unwrap();
    }

    get_connection().unwrap().execute(
        "INSERT INTO customers (name, balance) VALUES (?1, ?2)",
        params![customer.name, starting_balance],
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

pub fn update_balance(id: u16, balance: f64) -> Result<()> {
    let conn = get_connection().unwrap();

    conn.execute(
        &format!("UPDATE customers SET balance = ?1 WHERE id = ?2"),
        params![balance, id],
    )
    .unwrap();
    Ok(())
}

pub fn edit_customer(id: u16, customer: models::CustomerEdit) -> Result<()> {
    let conn = get_connection().unwrap();

    conn.execute(
        &format!("UPDATE customers SET name = ?1 WHERE id = ?2"),
        params![customer.name, id],
    )
    .unwrap();
    Ok(())
}

pub fn get_all_customers() -> Result<Vec<Customer>> {
    let mut record_list: Vec<Customer> = Vec::new();

    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM customers")?;

    stmt.query_map(params![], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
        })
    })?
    .for_each(|i| record_list.push(i.unwrap()));

    Ok(record_list)
}
