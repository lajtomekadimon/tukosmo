
CREATE OR REPLACE FUNCTION sc_posts_f_untranslated(
    language_of_user BIGINT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COUNT(*)
FROM t_posts

LEFT JOIN t_post_translations
ON tp_id = tpt_post
    AND tpt_lang = language_of_user

WHERE tpt_post IS NULL

$$;
