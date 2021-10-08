
CREATE OR REPLACE FUNCTION e_is_email(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

-- This is case insensitive (~ for sensitive)
SELECT text_value ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'

$$;
