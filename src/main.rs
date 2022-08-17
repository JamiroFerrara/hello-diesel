extern crate diesel;

use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use schema::Track::dsl::*;
    let connection = db::establish_connection();
    let results = hello_diesel::schema::track.load::<Track>(&connection) .expect("Error loading tracks");
}
