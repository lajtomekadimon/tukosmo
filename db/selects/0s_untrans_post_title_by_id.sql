
CREATE OR REPLACE FUNCTION s_untrans_post_title_by_id(
    post_id BIGINT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tp_title
FROM t_post_ids

INNER JOIN t_posts
ON tpi_id = tp_post

ORDER BY CASE
    WHEN s_language_id_by_code('en') IS NULL
    THEN 2
    ELSE CASE
        WHEN tp_lang = s_language_id_by_code('en')
        THEN 1
        ELSE 2
    END
END

LIMIT 1

$$;
