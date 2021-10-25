
CREATE OR REPLACE FUNCTION c_user_name_by_user_lang(
    user_id BIGINT,
    lang_id BIGINT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM t_user_names
    WHERE tun_user = user_id
        AND tun_lang = lang_id
    LIMIT 1
)

$$;
