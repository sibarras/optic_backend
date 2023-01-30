use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use std::time::SystemTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct PathBigID {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PathID {
    pub id: i32,
}


#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "sale_kinds")]
pub struct SaleKind {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "frame_brands")]
pub struct FrameBrand {
    pub id: Option<i32>,
    pub serial: i64,
    pub name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "sales")]
pub struct Sale {
    pub id: Option<i64>,
    pub date: Option<SystemTime>,
    pub sale_kind_id: i32,
    pub frame_brand_id: Option<i32>,
    pub quantity: i32,
}
