#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::Redirect;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
#[derive(Serialize)]
struct Context {
	food: String,
}

#[get("/")]
fn index() -> Redirect {
	Redirect::to("/BAGEL")
}

#[get("/eat/<food>")]
fn eat(food: String) -> Template {
	let context = Context{
		food: food
	};

	Template::render("index",&context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, eat])
    .attach(Template::fairing())
    .launch();
} 