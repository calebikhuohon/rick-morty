use std::{io};

pub fn connect() -> redis::Connection {
    let redis_conn_url = "redis://127.0.0.1:6399";

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub fn set<T: serde::Serialize>(conn: &mut redis::Connection, key: &str, val: T) -> Option<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(serde_json::to_string(&val).unwrap())
        .query(conn)
        .expect(&format!("failed to execute SET operation {}", key))
}

pub fn get<T>(conn: &mut redis::Connection, key: &str) -> Result<String, io::Error> {
    let result = redis::cmd("GET")
        .arg(key)
        .query(conn)
        .expect(&format!("failed to execute GET operation for key {}", key));

    Ok(result)
}
