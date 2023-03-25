#[macro_use]
extern crate rocket;

mod todo;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(todo::stage())
}
