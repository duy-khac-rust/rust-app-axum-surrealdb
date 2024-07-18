use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use domain_backend::student::{GetRequestStudent, Student, StudentInfo};
use inftra_backend::student::{
    create_student_json, delete_student_id, get_student_by_id, update_student_json, views_student,
};

use crate::SC;

pub fn student_route() -> Router<SC> {
    Router::new()
        .merge(get_students_route())
        .merge(get_student_route())
        .merge(create_student_route())
        .merge(update_student_route())
        .merge(delete_student_route())
}

pub fn get_students_route() -> Router<SC> {
    pub async fn get_students(State(db): State<SC>) -> Json<Vec<Student>> {
        views_student(db).await.unwrap()
    }

    Router::new().route("/", get(get_students))
}

pub fn get_student_route() -> Router<SC> {
    pub async fn get_student(
        State(db): State<SC>,
        Path(id): Path<GetRequestStudent>,
    ) -> Json<Student> {
        get_student_by_id(db, Path(id)).await
    }

    Router::new().route("/search/:id", get(get_student))
}

pub fn create_student_route() -> Router<SC> {
    pub async fn create_student(
        State(db): State<SC>,
        Json(student): Json<StudentInfo>,
    ) -> axum::http::StatusCode {
        create_student_json(db, Json(student)).await.unwrap()
    }

    Router::new().route("/create", post(create_student))
}

pub fn update_student_route() -> Router<SC> {
    pub async fn update_student(
        State(db): State<SC>,
        Path(id): Path<GetRequestStudent>,
        Json(student): Json<StudentInfo>,
    ) -> axum::http::StatusCode {
        update_student_json(db, Path(id), Json(student))
            .await
            .unwrap()
    }

    Router::new().route("/update/:id", put(update_student))
}

pub fn delete_student_route() -> Router<SC> {
    pub async fn delete_student(
        State(db): State<SC>,
        Path(id): Path<GetRequestStudent>,
    ) -> axum::http::StatusCode {
        delete_student_id(db, Path(id)).await.unwrap()
    }

    Router::new().route("/delete/:id", delete(delete_student))
}
