DELETE FROM testing.frame_brands
WHERE frame_brands.id = $1
RETURNING $table_fields;