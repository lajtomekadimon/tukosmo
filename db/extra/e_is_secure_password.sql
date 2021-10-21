
CREATE OR REPLACE FUNCTION e_is_secure_password(
    text_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT
PARALLEL SAFE

AS $$

SELECT (LENGTH(text_value) >= 8)
-- TODO: upper + lower + symbols + etc.
-- TODO: Remember that "bcrypt has a maximum length input length of 72 bytes
--       for most implementations".

$$;
