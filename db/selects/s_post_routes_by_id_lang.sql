
CREATE OR REPLACE FUNCTION s_post_routes_by_id_lang(
    post_id BIGINT,
    language_of_user BIGINT
)

RETURNS "RouteDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$


SELECT ARRAY(
    SELECT (
        s_language_by_id_lang(
            tpt_lang,
            language_of_user
        ),
        '/blog/' || tpt_permalink
    )::"RouteDB"
    FROM t_post_translations
    WHERE tpt_post = post_id
)

$$;
