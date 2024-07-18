use axum::{extract::Path, http::StatusCode, Json};
use domain_backend::student::{GetRequestStudent, Student, StudentInfo};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::error::ResultInftra;

pub mod error;

pub async fn views_student(db: Surreal<Client>) -> ResultInftra<Json<Vec<Student>>> {
    let data: Vec<Student> = db.select("student").await?;

    Ok(Json(data))
}

pub async fn get_student_by_id(
    db: Surreal<Client>,
    Path(path): Path<GetRequestStudent>,
) -> Json<Student> {
    let query = format!("SELECT * FROM student WHERE id = {}", path.id.to_string());
    match db.query(query).await {
        Ok(mut data) => {
            let student = data.take::<Option<Student>>(0).unwrap();
            Json(student.unwrap())
        }
        Err(err) => {
            eprintln!("Error fetching student by id: {}", err);
            return Json(Student::default());
        }
    }
}

pub async fn create_student_json(
    db: Surreal<Client>,
    Json(student): Json<StudentInfo>,
) -> ResultInftra<StatusCode> {
    let _: Vec<StudentInfo> = db.create("student").content(student).await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_student_json(
    db: Surreal<Client>,
    Path(id): Path<GetRequestStudent>,
    Json(student): Json<StudentInfo>,
) -> ResultInftra<StatusCode> {
    let _: Option<Student> = db
        .update(("student", id.id.to_string()))
        .content(student)
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_student_id(
    db: Surreal<Client>,
    Path(id): Path<GetRequestStudent>,
) -> ResultInftra<StatusCode> {
    let _: Option<Student> = db.delete(("student", id.id.to_string())).await?;

    Ok(StatusCode::OK)
}
