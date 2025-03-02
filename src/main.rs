use dotenv::dotenv;
use std::env;
use mysql::*;
use mysql::prelude::*;
use simple_logger::SimpleLogger;
use log::{LevelFilter};

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let table_name:&str;
    let column_name:&str;

    match args.len() {
        3 => {
            table_name = args[1].trim().as_ref();
            column_name = args[2].trim().as_ref();
        },
        _ => {
            log::info!("Wrong number of arguments. Expected [table_name] [column_name]");
            return
        }
    }

    let database_host = env::var("DATABASE_SOURCE_HOST");
    let database_name = env::var("DATABASE_SOURCE_NAME");
    let database_user = env::var("DATABASE_SOURCE_USER");
    let database_password = env::var("DATABASE_SOURCE_PASSWORD");

    let database_connection_string = format!(
        "mysql://{}:{}@{}:3306/{}",
        database_user.unwrap(),
        database_password.unwrap(),
        database_host.unwrap(),
        database_name.unwrap()
    );

    let pool = Pool::new(database_connection_string).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let alter_query:String = "ALTER TABLE :table_name MODIFY :column_name BIGINT".replace(":table_name", table_name).replace(":column_name", column_name);

    match conn.query_drop(alter_query) {
        Err(e) => {
            log::error!("An error occurred when updating column for {:?}: {:?}", table_name, e);
        },
        _ => {
            log::info!("Table {:?} - UPDATED", table_name)
        }
    }
}
