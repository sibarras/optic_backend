UPDATE testing.sales
SET sale_kind_id = $2,
    frame_brand_id = $3,
    quantity = $4
WHERE sales.id = $1
RETURNING $table_fields;