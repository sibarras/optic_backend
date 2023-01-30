INSERT INTO testing.frame_brands(serial, name)
VALUES ($1, $2)
RETURNING $table_fields;