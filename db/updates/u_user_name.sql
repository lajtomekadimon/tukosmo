
CREATE OR REPLACE FUNCTION u_user_name(
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

    UPDATE t_user_names
    SET tun_name = name_value,
        tun_date = NOW()
    WHERE tun_id = user_id
        AND tun_lang = lang_id
    RETURNING tun_id INTO user_name_id;

    RETURN user_name_id;

END;

$$;
