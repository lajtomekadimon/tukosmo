
CREATE OR REPLACE FUNCTION sc_posts(
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
    AND CASE
        WHEN tpt_lang IS NULL
        THEN TRUE
        ELSE tpt_lang = language_of_user
    END
    AND NOT tpt_deleted

$$;
