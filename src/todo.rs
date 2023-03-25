use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

type Id = usize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Todo {
    id: Option<Id>,
    title: String,
    completed: bool,
}

type TodoList = Mutex<Vec<Todo>>;
type Todos<'r> = &'r State<TodoList>;

#[post("/", format = "json", data = "<title>")]
async fn new(title: String, list: Todos<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    let todo = Todo {
        id: Some(id),
        title: title,
        completed: false,
    };
    list.push(todo);
    json!({"status": "ok", "id": id})
}

// TODO: GET /todos
// TODO: GET /todo/<id>
// TODO: PUT /todo/<id>

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("TODO", |rocket| async {
        rocket
            .mount("/todo", routes![new])
            .manage(TodoList::new(vec![]))
    })
}
