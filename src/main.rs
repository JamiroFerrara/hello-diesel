extern crate diesel;

use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use schema::Track::dsl::*;

    let connection = db::establish_connection();
}
