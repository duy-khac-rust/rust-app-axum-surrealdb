use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};


#[derive(Deserialize, Serialize)]
pub struct Student {
    pub id: Thing,
    pub name: String,
    pub age: u8,
    pub active: bool,

}

impl Default for Student {
    fn default() -> Self {
        Student {
            id: Thing {
                id: Id::String("not found".to_string()),
                tb: "no found".to_string()
            },
            name: String::new(),
            age: 0,
            active: false,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct StudentInfo {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

#[derive(Deserialize, Serialize)]
pub struct GetRequestStudent {
    pub id: String,
}
