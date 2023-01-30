use crate::{errors::MyError, models::FrameBrand};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_frame_brand(client: &Client, frame_brand_info: FrameBrand) -> Result<FrameBrand, MyError> {

    let _stmt = include_str!("../../sql/frame_brands/add_frame_brand.sql");
    let _stmt = _stmt.replace("$table_fields", &FrameBrand::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    
    client
        .query(
            &stmt,
            &[
                &frame_brand_info.serial,
                &frame_brand_info.name,
            ],
        )
        .await?
        .iter()
        .map(|row| FrameBrand::from_row_ref(row).unwrap())
        .collect::<Vec<FrameBrand>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_frame_brand(client: &Client, id: i32) -> Result<FrameBrand, MyError> {
    let _stmt = include_str!("../../sql/frame_brands/get_frame_brand.sql");
    let _stmt = _stmt.replace("$table_fields", &FrameBrand::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    
    FrameBrand::from_row_ref(
        &client
            .query_opt(
                &stmt, 
                &[
                    &id
                ]
            )
            .await?
            .ok_or(MyError::NotFound)?
        )
        .map_err(MyError::PGMError)
}

pub async fn get_frame_brands(client: &Client) -> Result<Vec<FrameBrand>, MyError> {
    let _stmt = include_str!("../../sql/frame_brands/get_frame_brands.sql");
    let _stmt = _stmt.replace("$table_fields", &FrameBrand::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    Ok(
        client
            .query(
                &stmt,
                &[]
            )
            .await?
            .iter()
            .map(|row| FrameBrand::from_row_ref(row).unwrap())
            .collect::<Vec<_>>()
    )
}

pub async fn delete_frame_brand(client: &Client, id: i32) -> Result<FrameBrand, MyError> {
    let _stmt = include_str!("../../sql/frame_brands/delete_frame_brand.sql");
    let _stmt = _stmt.replace("$table_fields", &FrameBrand::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &id
            ]
        )
        .await?
        .iter()
        .map(|row| FrameBrand::from_row_ref(row).unwrap())
        .collect::<Vec<FrameBrand>>()
        .pop().ok_or(MyError::NotFound)
}

pub async fn update_frame_brand(client: &Client, sale_info: FrameBrand) -> Result<FrameBrand, MyError> {
    let _stmt = include_str!("../../sql/frame_brands/update_frame_brand.sql");
    let _stmt = _stmt.replace("$table_fields", &FrameBrand::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &sale_info.id,
                &sale_info.serial,
                &sale_info.name,
            ]
        )
        .await?
        .iter()
        .map(|row| FrameBrand::from_row_ref(row).unwrap())
        .collect::<Vec<_>>()
        .pop().ok_or(MyError::NotFound)
}