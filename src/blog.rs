use axum::extract::Path;
use axum::http::StatusCode;
use axum::{response::IntoResponse, extract::State, Json};
use serde::{ Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Blog {
    id: i32,
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateBlog {
    title: String,
    description: String,
}

#[derive(Deserialize)]
pub struct BlogId{
    id: i32
}

pub struct BlogCtrl;

impl BlogCtrl {
    pub async fn get_all_blogs(State(db): State<PgPool>) -> impl IntoResponse {
        let res = sqlx::query_as::<_, Blog>("SELECT * FROM blog")
            .fetch_all(&db)
            .await;

        match res {
            Ok(todos) => (StatusCode::OK, Json(todos).into_response()),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string().into_response())
        }
    }
    
    pub async fn create_blog(State(db): State<PgPool>, Json(data): Json<CreateBlog>) -> impl IntoResponse {
        let res = sqlx::query_as::<_, Blog>("INSERT INTO blog (title, description) VALUES ($1, $2) RETURNING *")
            .bind(&data.title)
            .bind(&data.description)
            .fetch_one(&db)
            .await;
    
        match res {
            Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }

    pub async fn delete_blog(State(db): State<PgPool>, Path(del): Path<BlogId> ) -> impl IntoResponse {
       let res = sqlx::query("DELETE FROM blog WHERE id = $1" )
            .bind(del.id)
            .execute(&db)
            .await;

       match res {
            Ok(_) => (StatusCode::OK, Json(json!({ "msg": "del success" }))).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }  
    }

    pub async fn update_blog( State(db): State<PgPool>, Path(upd): Path<BlogId>, Json(data): Json<CreateBlog> ) -> impl IntoResponse {
        let query = "UPDATE blog SET title = $1, description = $2 WHERE id = $3 RETURNING *";
    
        let res = sqlx::query_as::< _, Blog>(query)
            .bind(&data.title)
            .bind(&data.description)
            .bind(upd.id)
            .fetch_one(&db)
            .await;
    
        match res {
            Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }

}