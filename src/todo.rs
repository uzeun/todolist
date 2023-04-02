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

#[derive(Serialize, Deserialize)]
struct NewTodo {
    title: String,
}

type TodoList = Mutex<Vec<Todo>>;
type Todos<'r> = &'r State<TodoList>;

#[post("/", format = "json", data = "<new_todo>")]
async fn new(new_todo: Json<NewTodo>, list: Todos<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    let todo = Todo {
        id: Some(id),
        title: new_todo.title.to_string(),
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

#[put("/<id>", format = "json", data = "<new_todo>")]
async fn edit_title(id: Id, new_todo: Json<NewTodo>, list: Todos<'_>) -> Option<Value> {
    match list.lock().await.get_mut(id) {
        Some(existing) => {
            let title_ref = &mut existing.title;
            *title_ref = new_todo.title.to_string();
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
