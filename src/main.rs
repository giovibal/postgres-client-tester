use std::env;
use log::{debug, error, info};
use postgres::{Client, NoTls};

fn main() -> Result<(), postgres::Error> {
    env_logger::init();
    info!("log level: info");
    debug!("log level: debug");
    error!("log level: error");

    // let dbconn = env!("DB_CONN", "Missing `DB_CONN` env variable. ");
    let dbconn = env::var("DB_CONN").unwrap_or("postgresql://postgres:Passw0rd@localhost/tests".to_string());
    info!("db connection string: {}", dbconn);

    let mut client = Client::connect("postgresql://postgres:Passw0rd@localhost/tests", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS appen_data  (
            id              SERIAL PRIMARY KEY,
            val             VARCHAR NOT NULL
            )
    ")?;

    Ok(())
}
