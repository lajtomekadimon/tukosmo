
CREATE OR REPLACE FUNCTION sc_posts_by_pref_lang(
    language_of_user BIGINT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT DISTINCT ON (tp_id) COUNT(*)
FROM t_posts

INNER JOIN t_post_translations
ON tp_id = tpt_post
    AND (NOT tpt_deleted)
    AND (NOT tpt_draft)

GROUP BY tp_id

$$;
