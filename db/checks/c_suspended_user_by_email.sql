
CREATE OR REPLACE FUNCTION c_suspended_user_by_email(
    email_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tu_suspended
FROM t_users
WHERE tu_email = email_value
LIMIT 1

$$;
