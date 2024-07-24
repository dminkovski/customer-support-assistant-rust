```rust
use actix_cors::Cors;

use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: u64,
    status: String,
    user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
    orders: HashMap<u64, Order>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
            orders: HashMap::new(),
        }
    }
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    fn get(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }
    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    fn delete(&mut self, id: u64) {
        self.tasks.remove(&id);
    }
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    fn insert_order(&mut self, order: Order) {
        self.orders.insert(order.id, order);
    }
    fn get_order(&self, id: u64) -> Option<&Order> {
        self.orders.get(&id)
    }
    fn update_order(&mut self, order: Order) {
        self.orders.insert(order.id, order);
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn get_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let all_tasks = db.get_all();
    HttpResponse::Ok().json(all_tasks)
}

async fn get_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    match db.get(id.into_inner()) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete(id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn check_order_status(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_order(id.into_inner()) {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn contact_shipping_carrier() -> impl Responder {
    HttpResponse::Ok().body("Contacting shipping carrier...")
}

async fn issue_refund(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    if db.orders.contains_key(&id.into_inner()) {
        HttpResponse::Ok().body("Refund issued")
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn resend_order(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    if db.orders.contains_key(&id.into_inner()) {
        HttpResponse::Ok().body("Order resent")
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn provide_discount() -> impl Responder {
    HttpResponse::Ok().body("Discount provided on next purchase")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONAL"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(get_all_tasks))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            .route("/register", web::post().to(register))
            .route("/order/status/{id}", web::get().to(check_order_status))
            .route("/order/contact", web::post().to(contact_shipping_carrier))
            .route("/order/refund/{id}", web::post().to(issue_refund))
            .route("/order/resend/{id}", web::post().to(resend_order))
            .route("/order/discount", web::post().to(provide_discount))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```