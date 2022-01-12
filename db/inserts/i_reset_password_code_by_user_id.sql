
CREATE OR REPLACE FUNCTION i_reset_password_code_by_user_id(
    user_id BIGINT
)

RETURNS UUID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    reset_password_code_id UUID;

    date_value TIMESTAMPTZ;
    exp_date_value TIMESTAMPTZ;

BEGIN

    reset_password_code_id := GEN_RANDOM_UUID();

    date_value := NOW();
    exp_date_value := date_value + (20 * INTERVAL '1 minute');

    INSERT INTO t_reset_password_codes (
        trpc_id,
        trpc_user,
        trpc_date,
        trpc_expiration_date
    ) VALUES (
        reset_password_code_id,
        user_id,
        date_value,
        exp_date_value
    );

    RETURN reset_password_code_id;

END;

$$;
