
CREATE OR REPLACE FUNCTION e_is_filename(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

-- This is case sensitive (~* for insensitive)
SELECT text_value ~ '^[A-Za-z0-9._-]+$' AND
    (LENGTH(text_value) > 0) AND
    (LENGTH(text_value) < 251) AND
    text_value <> '.'

$$;
