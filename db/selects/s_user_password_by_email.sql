
CREATE OR REPLACE FUNCTION s_user_password_by_email(
    email_value TEXT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tu_password
FROM t_users
WHERE tu_email = email_value
LIMIT 1

$$;
