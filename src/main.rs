use std::{env, thread};
use std::time::Duration;
use log::{debug, error, info};
use postgres::{Client, Config, NoTls};
use postgres::types::{Oid, Type};
use clap::Parser;
use r2d2_postgres::PostgresConnectionManager;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct MyArgs {
    /// Number of milliseconds to sleep between queries
    #[clap(short, long, default_value_t = 1000)]
    sleep: u64,

    /// Choose query type: 0 both, 1 insert, 2 select
    #[clap(short, long, default_value_t = 0)]
    querytype: usize,

}

fn main() -> Result<(), postgres::Error> {
    env_logger::init();
    info!("log level: info");
    debug!("log level: debug");
    error!("log level: error");

    let myargs: MyArgs = MyArgs::parse();
    info!("sleep: {}", myargs.sleep);


    // let dbconn = env!("DB_CONN", "Missing `DB_CONN` env variable. ");
    let dbconn = env::var("DB_CONN").expect("DB_CONN env variable");
    info!("db connection string: {}", dbconn);

    // let mut client = Client::connect(dbconn.as_str(), NoTls)?;

    // let mut config: Config = Config::from_str(dbconn.as_str()).expect("Connection url as: postgresql://postgres:Passw0rd@localhost:5432/tests");
    let mut config: Config = dbconn.as_str().parse::<Config>().expect("Connection url as: postgresql://postgres:Passw0rd@localhost:5432/tests");
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = r2d2::Pool::new(manager).unwrap();

    // get a client
    let pool = pool.clone();
    let mut client = pool.get().unwrap();

    // prepare DB: create a test table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS test_table  (
            id              SERIAL PRIMARY KEY,
            val             VARCHAR NOT NULL
            )
    ")?;

    let mut counter : u128 = 0;
    loop {

        // get a new client
        let pool = pool.clone();
        let mut client = pool.get().expect("a client from pool");

        match myargs.querytype {
            1 => {
                let query = "INSERT INTO test_table (val) VALUES ($1)";
                let v = format!("rec n. {}", counter);
                let result = client.execute(query, &[&v]);
                if result.is_err() {
                    let err = result.err().unwrap();
                    error!("{}", err.to_string());
                    //return Result::Err(err);
                } else {
                    let cnt = result.unwrap();
                    info!("writed: {:?}", cnt);
                }
            },
            2 => {
                let result = client.query_one("SELECT count(*) FROM test_table", &[]);
                if result.is_err() {
                    let err = result.err().unwrap();
                    error!("{}", err.to_string());
                    // return Result::Err(err);
                } else {
                    let cnt = result.unwrap();
                    let cnt: i64 = cnt.get(0);
                    info!("result: {:?}", cnt);
                }
            },
            _ => {
                let query = "INSERT INTO test_table (val) VALUES ($1)";
                let v = format!("rec n. {}", counter);
                let result = client.execute(query, &[&v]);
                if result.is_err() {
                    let err = result.err().unwrap();
                    error!("{}", err.to_string());
                    // return Result::Err(err);
                }

                let result = client.query_one("SELECT count(*) FROM test_table", &[]);
                if result.is_err() {
                    let err = result.err().unwrap();
                    error!("{}", err.to_string());
                    // return Result::Err(err);
                } else {
                    let cnt = result.unwrap();
                    let cnt: i64 = cnt.get(0);
                    info!("result: {:?}", cnt);
                }
            },
        }

        counter = counter +1;
        thread::sleep(Duration::from_millis(myargs.sleep));
    }

    Ok(())
}
