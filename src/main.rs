mod env;
mod util;
mod route;

use rocket::{ launch, routes };

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    rocket::build()
        .mount("/", routes![route::login, route::callback])
}
