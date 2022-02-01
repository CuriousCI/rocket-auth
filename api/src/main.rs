#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::user::User;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::response::status::Unauthorized;
use rocket::response::Redirect;
use rocket_sync_db_pools::database;
use std::path::Path;

#[database("cookies")]
pub struct Database(diesel::PgConnection);

#[post("/request/login", data = "<user>")]
async fn login_request(
	user: Form<User>,
	connection: Database,
	cookies: &CookieJar<'_>,
) -> Result<Redirect, Unauthorized<()>> {
	let user = connection
		.run(move |connection| {
			use schema::users::dsl::*;
			users
				.filter(email.eq(user.email.clone()))
				.filter(password.eq(user.password.clone()))
				.first::<User>(connection)
		})
		.await;

	match user {
		Ok(user) => {
			cookies.add(Cookie::build("user_id", user.id.to_string()).finish());
			Ok(Redirect::to(uri!("/")))
		}
		_ => Err(Unauthorized(None)),
	}
}

#[get("/")]
async fn index(_guard: User) -> NamedFile {
	NamedFile::open(Path::new("/app/public/index.html"))
		// NamedFile::open(Path::new("D:/school/sistemi/cookies/public/index.html"))
		.await
		.ok()
		.unwrap()
}

#[get("/", rank = 2)]
fn login_redirect() -> Redirect {
	Redirect::to(uri!("/login"))
}

#[get("/email")]
async fn email(user: User) -> String {
	user.email
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Database::fairing())
		// .mount("/", FileServer::from("D:/school/sistemi/cookies/public"))
		.mount("/", FileServer::from("/app/public"))
		.mount("/", routes![index, login_redirect, login_request, email])
}
