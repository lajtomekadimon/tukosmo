
CREATE OR REPLACE FUNCTION e_is_permalink(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT (text_value ~ '^[a-z-]+$') AND
    (LENGTH(text_value) > 0) AND
    (LENGTH(text_value) < 600)

$$;
