extern crate diesel;

#[macro_use] extern crate rocket;

use self::diesel::prelude::*;
use hello_diesel::models::*;

mod db;

#[launch]
async fn rocket() -> _ {
    #[get("/Artists")]
    async fn artists() -> String {
        use hello_diesel::schema::Artist::dsl::*;

        let connection = db::establish_connection();
        let artists = Artist.load::<hello_diesel::models::Artist>(&connection);
        match artists {
            Ok(artists) => format!("{:?}", artists),
            Err(e) => format!("Error: {}", e),
        }
    }

    rocket::build()
        .mount("/", routes![artists])
}
