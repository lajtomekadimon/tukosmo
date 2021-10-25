
CREATE OR REPLACE FUNCTION d_user_name_by_user_lang(
    user_id BIGINT,
    lang_id BIGINT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_user_names
    WHERE tun_user = user_id
        AND tun_lang = lang_id;

END;

$$;
