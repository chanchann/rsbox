/*
1. 创建新会话（POST /session）：生成一个新的UUID作为会话ID，并存储一些会话数据。

2.获取会话信息（GET /session/{id}）：根据会话ID返回会话数据。

3.删除会话（DELETE /session/{id}）：根据会话ID删除会话（模拟登出）。

# 创建新会话
curl -X POST http://localhost:8080/session

# 获取会话信息（用上一步返回的UUID替换{uuid}）
curl http://localhost:8080/session/{uuid}

# 删除会话
curl -X DELETE http://localhost:8080/session/{uuid}
*/

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// 用于存储会话数据的结构
#[derive(Serialize, Deserialize, Clone)]
struct SessionData {
    user_id: String,
    login_time: String,
}

// 应用程序状态，用于存储所有活跃会话
struct AppState {
    sessions: Mutex<HashMap<String, SessionData>>,
}

// 创建新会话
async fn create_session(data: web::Data<AppState>) -> impl Responder {
    let session_id = Uuid::new_v4().to_string();
    let session_data = SessionData {
        user_id: "user123".to_string(),  // 在实际应用中，这应该是从身份验证过程中获得的
        login_time: chrono::Utc::now().to_rfc3339(),
    };

    let mut sessions = data.sessions.lock().unwrap();
    sessions.insert(session_id.clone(), session_data.clone());

    HttpResponse::Ok().json(session_id)
}

// 获取会话信息
async fn get_session(session_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let sessions = data.sessions.lock().unwrap();
    match sessions.get(&session_id.into_inner()) {
        Some(session_data) => HttpResponse::Ok().json(session_data),
        None => HttpResponse::NotFound().body("Session not found"),
    }
}

// 删除会话（登出）
async fn delete_session(session_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut sessions = data.sessions.lock().unwrap();
    match sessions.remove(&session_id.into_inner()) {
        Some(_) => HttpResponse::Ok().body("Session deleted"),
        None => HttpResponse::NotFound().body("Session not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sessions: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/session", web::post().to(create_session))
            .route("/session/{id}", web::get().to(get_session))
            .route("/session/{id}", web::delete().to(delete_session))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}