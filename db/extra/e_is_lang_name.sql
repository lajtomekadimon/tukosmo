
CREATE OR REPLACE FUNCTION e_is_lang_name(
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
    (POSITION('  ' IN text_value) = 0)

$$;
