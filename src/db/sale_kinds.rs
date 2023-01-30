use crate::{errors::MyError, models::SaleKind};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_sale_kinds(client: &Client) -> Result<Vec<SaleKind>, MyError> {
    let _stmt = include_str!("../../sql/sale_kinds/get_sale_kinds.sql");
    let _stmt = _stmt.replace("$table_fields", &SaleKind::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| SaleKind::from_row_ref(row).unwrap())
        .collect::<Vec<SaleKind>>())
}

pub async fn get_sale_kind(client: &Client, id: i32) -> Result<SaleKind, MyError> {
    let _stmt = include_str!("../../sql/sale_kinds/get_sale_kind.sql");
    let _stmt = _stmt.replace("$table_fields", &SaleKind::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    SaleKind::from_row_ref(
        &client
            .query_opt(&stmt, &[&id])
            .await?
            .ok_or(MyError::NotFound)?,
    )
    .map_err(MyError::PGMError)
}
