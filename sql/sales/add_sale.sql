INSERT INTO testing.sales(sale_kind_id, frame_brand_id, quantity)
VALUES ($1, $2, $3)
RETURNING $table_fields;