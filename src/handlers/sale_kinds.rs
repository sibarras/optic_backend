use crate::db::sale_kinds;
use crate::errors::MyError;
use crate::models::PathID;
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn get_sale_kinds(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let sale_kinds = sale_kinds::get_sale_kinds(&client).await?;

    Ok(HttpResponse::Ok().json(sale_kinds))
}

pub async fn get_sale_kind(
    path_id: web::Path<PathID>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let sale_kind = sale_kinds::get_sale_kind(&client, path_id.id).await?;

    Ok(HttpResponse::Ok().json(sale_kind))
}
