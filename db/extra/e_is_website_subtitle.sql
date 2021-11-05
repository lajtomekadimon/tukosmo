
CREATE OR REPLACE FUNCTION e_is_website_subtitle(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT (LEFT(text_value, 1) <> ' ') AND
    (RIGHT(text_value, 1) <> ' ') AND
    (LENGTH(text_value) > 0) AND
    (LENGTH(text_value) < 400)

$$;
