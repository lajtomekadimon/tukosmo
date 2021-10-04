
CREATE OR REPLACE FUNCTION sc_posts_by_lang(
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

INNER JOIN t_post_translations
ON tp_id = tpt_post
    AND tpt_lang = language_of_user
    AND (NOT tpt_deleted)
    AND (NOT tpt_draft)

$$;
