
CREATE OR REPLACE FUNCTION s_untrans_post_title_by_id(
    post_id BIGINT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tpt_title
FROM t_posts

INNER JOIN t_post_translations
ON tp_id = tpt_post
    AND tpt_post = post_id

ORDER BY CASE
    WHEN s_language_id_by_code('en') IS NULL
    THEN 2
    ELSE CASE
        WHEN tpt_lang = s_language_id_by_code('en')
        THEN 1
        ELSE 2
    END
END

LIMIT 1

$$;
