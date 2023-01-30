use crate::errors::MyError;
use crate::{db::users, models::User};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = users::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
