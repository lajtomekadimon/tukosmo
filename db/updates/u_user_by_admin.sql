
CREATE OR REPLACE FUNCTION u_user_by_admin(
    user_id BIGINT,
    email_value TEXT,
    name_value TEXT,
    is_admin BOOL,
    is_suspended BOOL
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_users
    SET tu_email = email_value,
        tu_name = name_value,
        tu_admin = is_admin,
        tu_suspended = is_suspended
    WHERE tu_id = user_id;

    RETURN user_id;

END;

$$;
