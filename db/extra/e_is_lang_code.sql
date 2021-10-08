
CREATE OR REPLACE FUNCTION e_is_lang_code(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT (text_value ~ '^[a-z]+$' AND CHAR_LENGTH(text_value) = 2)
    OR (
        SUBSTRING(text_value FOR 2) ~ '^[a-z]+$' AND
        SUBSTRING(text_value FROM 3 FOR 3) = '-' AND
        SUBSTRING(text_value FROM 4) ~ '^[a-z]+$' AND
        CHAR_LENGTH(text_value) = 5
    )

$$;
