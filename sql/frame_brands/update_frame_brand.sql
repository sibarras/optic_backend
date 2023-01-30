UPDATE testing.frame_brands
SET (serial, name) = ($2, $3)
WHERE frame_brands.id = $1
RETURNING $table_fields;