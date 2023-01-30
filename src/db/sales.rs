use crate::{errors::MyError, models::Sale};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_sale(client: &Client, sale_info: Sale) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sales/add_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &sale_info.sale_kind_id,
                &sale_info.frame_brand_id,
                &sale_info.quantity,
            ],
        )
        .await?
        .iter()
        .map(|row| Sale::from_row_ref(row).unwrap())
        .collect::<Vec<Sale>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_sale(client: &Client, id: i64) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sales/get_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| Sale::from_row_ref(row).unwrap())
        .collect::<Vec<Sale>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_sales(client: &Client) -> Result<Vec<Sale>, MyError> {
    let _stmt = include_str!("../../sql/sales/get_sales.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Sale::from_row_ref(row).unwrap())
        .collect::<Vec<Sale>>())
}

pub async fn delete_sale(client: &Client, id: i64) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sales/delete_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| Sale::from_row_ref(row).unwrap())
        .collect::<Vec<Sale>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn update_sale(client: &Client, sale_info: Sale) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sales/update_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &sale_info.id,
                &sale_info.sale_kind_id,
                &sale_info.frame_brand_id,
                &sale_info.quantity,
            ],
        )
        .await?
        .iter()
        .map(|row| Sale::from_row_ref(row).unwrap())
        .collect::<Vec<Sale>>()
        .pop()
        .ok_or(MyError::NotFound)
}
