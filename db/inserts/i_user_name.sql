
CREATE OR REPLACE FUNCTION i_user_name(
    user_id BIGINT,
    name_value TEXT,
    lang_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    user_name_id BIGINT;

BEGIN

    INSERT INTO t_user_names (
        --tun_id,
        tun_user,
        tun_name,
        tun_lang
        --tu_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        user_id,
        name_value,
        lang_id
        --NOW()
    ) RETURNING tun_id INTO user_name_id;

    RETURN user_name_id;

END;

$$;
