
CREATE OR REPLACE FUNCTION e_is_domain(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

-- This is case insensitive (~ for sensitive)
SELECT text_value ~* '^([a-z0-9|-]+\.)*[a-z0-9|-]+\.[a-z]+$'

$$;
