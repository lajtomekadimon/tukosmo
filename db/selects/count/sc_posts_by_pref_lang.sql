
CREATE OR REPLACE FUNCTION sc_posts_by_pref_lang(
    language_of_user BIGINT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COUNT(*)
FROM (
    SELECT DISTINCT ON (tp_id)
        tp_id

    FROM t_posts

    INNER JOIN t_post_translations
    ON tp_id = tpt_post
        --AND tpt_lang = language_of_user
        AND (NOT tpt_deleted)
        AND (NOT tpt_draft)

    INNER JOIN t_users a
    ON tpt_translator = a.tu_id

    INNER JOIN t_users b
    ON tp_author = b.tu_id

    ORDER BY tp_id, CASE
        WHEN tpt_lang = language_of_user THEN 1
        -- TODO: Add English between 1 and 2
        ELSE 2
    END ASC
) x

$$;
