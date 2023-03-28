use rocket::serde::json::{json, Value};
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

#[get("/", format = "json")]
async fn get_list(list: Todos<'_>) -> Value {
    let list = list.lock().await;
    json!(list.as_slice())
}

#[get("/<id>", format = "json")]
async fn get(id: Id, list: Todos<'_>) -> Option<Value> {
    let list = list.lock().await;
    let todo = list.get(id)?;
    Some(json!(todo))
}

#[put("/<id>", format = "json", data = "<title>")]
async fn edit_title(id: Id, title: String, list: Todos<'_>) -> Option<Value> {
    match list.lock().await.get_mut(id) {
        Some(existing) => {
            let title_ref = &mut existing.title;
            *title_ref = title;
            Some(json!({"status": "ok"}))
        }
        None => None,
    }
}

// TODO: PUT /todo/<id>/complete
// TODO: DELETE /todo/<id>

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("TODO", |rocket| async {
        rocket
            .mount("/todo", routes![new, get_list, get, edit_title])
            .manage(TodoList::new(vec![]))
    })
}
