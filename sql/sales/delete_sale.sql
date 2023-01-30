DELETE FROM testing.sales
WHERE sales.id = $1
RETURNING $table_fields;