use crate::db::sales;
use crate::errors::MyError;
use crate::models::{PathBigID, Sale};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_sale(
    sale: web::Json<Sale>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let sale_info: Sale = sale.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_sale = sales::add_sale(&client, sale_info).await?;

    Ok(HttpResponse::Created().json(new_sale))
}

pub async fn get_sales(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let sales = sales::get_sales(&client).await?;

    Ok(HttpResponse::Ok().json(sales))
}

pub async fn get_sale(
    path_id: web::Path<PathBigID>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let sale = sales::get_sale(&client, path_id.id).await?;

    Ok(HttpResponse::Ok().json(sale))
}

pub async fn delete_sale(
    path_id: web::Path<PathBigID>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let sale = sales::delete_sale(&client, path_id.id).await?;

    Ok(HttpResponse::Ok().json(sale.id))
}

pub async fn update_sale(
    path_id: web::Path<PathBigID>,
    sale_info: web::Json<Sale>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let sale_info: Sale = sale_info.into_inner();
    if sale_info.id.ok_or(MyError::BadRequest)? != path_id.id {
        return Err(MyError::BadRequest)?;
    }

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_sale: Sale = sales::update_sale(&client, sale_info).await?;

    Ok(HttpResponse::Ok().json(new_sale))
}
