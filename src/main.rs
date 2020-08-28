#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

pub mod applications;
pub mod schema;

use crate::applications::{Applications, NewApplications};

#[database("unipayv10")]
struct DbConn(diesel::MysqlConnection);

#[get("/info")]
fn read(conn: DbConn) -> Result<Json<Vec<Applications>>, String> {
    use crate::schema::dkp_applications::dsl::*;
    dkp_applications.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying heroes from the database".into()
    }).map(Json)
}

fn main() {
    rocket::ignite()
    .attach(DbConn::fairing())
    .mount("/app", routes![read])
    .launch();
}