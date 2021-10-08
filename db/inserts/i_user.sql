
CREATE OR REPLACE FUNCTION i_user(
    email_value TEXT,
    epassword_value TEXT,
    name_value TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    user_id BIGINT;

BEGIN

    INSERT INTO t_users (
        --tu_id,
        tu_email,
        tu_password,
        tu_name
        --tu_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        email_value,
        epassword_value,  -- previously: CRYPT('password', GEN_SALT('bf'))
        name_value
        --NOW()
    ) RETURNING tu_id INTO user_id;

    RETURN user_id;

END;

$$;
