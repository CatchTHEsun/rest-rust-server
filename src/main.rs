#![feature(decl_macro)]

#[macro_use]
extern create diesel;
#[macro_use]
extern create diesel_migrations;
#[macro_use]
extern create rocket;
#[macro_use]
extern create rocket_contrib;

use create::database::TimesheetsDatabaseInitialized;
use create::routes::TimesheetsRoutesInitialized;

mod routes;
pub mod handlers;
pub mod database;
mod config;
mod schema;

fn main() {
    rocket::custom(config::from_env())
        .manage_database()
        .mount_timesheet_routes()
        .launch();
}
