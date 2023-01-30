use crate::db::frame_brands;
use crate::errors::MyError;
use crate::models::{FrameBrand, PathID};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_frame_brand(
    frame_brand: web::Json<FrameBrand>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let frame_brand_info: FrameBrand = frame_brand.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_frame_brand = frame_brands::add_frame_brand(&client, frame_brand_info).await?;

    Ok(HttpResponse::Created().json(new_frame_brand))
}

pub async fn get_frame_brands(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let frame_brands = frame_brands::get_frame_brands(&client).await?;

    Ok(HttpResponse::Ok().json(frame_brands))
}

pub async fn get_frame_brand(
    path_id: web::Path<PathID>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let frame_brand = frame_brands::get_frame_brand(&client, path_id.id).await?;

    Ok(HttpResponse::Ok().json(frame_brand))
}

pub async fn delete_frame_brand(
    path_id: web::Path<PathID>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let frame_brand = frame_brands::delete_frame_brand(&client, path_id.id).await?;

    Ok(HttpResponse::Ok().json(frame_brand.id))
}

pub async fn update_frame_brand(
    path_id: web::Path<PathID>,
    frame_brand_info: web::Json<FrameBrand>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let frame_brand_info = frame_brand_info.into_inner();
    if frame_brand_info.id.ok_or(MyError::BadRequest)? != path_id.id {
        return Err(MyError::BadRequest)?;
    }

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_frame_brand = frame_brands::update_frame_brand(&client, frame_brand_info).await?;

    Ok(HttpResponse::Ok().json(new_frame_brand))
}
