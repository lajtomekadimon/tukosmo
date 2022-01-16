
CREATE OR REPLACE FUNCTION s_user_name_by_user_lang(
    user_id BIGINT,
    language_of_user BIGINT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tun_name
FROM t_user_names
WHERE tun_user = user_id
    AND tun_lang = language_of_user
LIMIT 1

$$;
