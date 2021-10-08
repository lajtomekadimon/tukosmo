
CREATE OR REPLACE FUNCTION e_is_lang_code(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT (text_value ~ '^[a-z]+$' AND LENGTH(text_value) = 2)
    OR (
        LEFT(text_value, 2) ~ '^[a-z]+$' AND
        SUBSTRING(text_value FROM 3 FOR 1) = '-' AND
        RIGHT(text_value, 2) ~ '^[a-z]+$' AND
        LENGTH(text_value) = 5
    )

$$;
