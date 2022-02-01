use crate::*;
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(FromForm, Queryable)]
pub struct User {
	pub id: i32,
	pub email: String,
	pub password: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
		let connection = try_outcome!(request.guard::<Database>().await);

		let user_id = request.cookies().get("user_id").map(|crumb| crumb.value());

		if user_id.is_none() {
			return Outcome::Forward(());
		}

		let user_id: i32 = user_id.unwrap().to_string().parse().unwrap();

		let user = connection
			.run(move |connection| {
				use schema::users::dsl::*;
				users.filter(id.eq(user_id)).first::<User>(connection)
			})
			.await;

		match user {
			Ok(value) => Outcome::Success(value),
			_ => Outcome::Forward(()),
		}
	}
}
