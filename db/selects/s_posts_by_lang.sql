
CREATE OR REPLACE FUNCTION s_posts_by_lang(
    language_of_user INT8
)

RETURNS TABLE(
    tp_id BIGINT,
    tp_title TEXT,
    tp_description TEXT,
    tp_body TEXT,
    tp_permalink TEXT,
    tp_author BIGINT,
    tp_date TEXT,
    tp_date_trans TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tpi_id AS tp_id,
    tp_title AS tp_title,
    tp_description AS tp_description,
    tp_body AS tp_body,
    tp_permalink AS tp_permalink,
    tp_author AS tp_author,
    tpi_date AS tp_date,
    tp_date AS tp_date_trans
FROM t_post_ids
INNER JOIN t_posts
ON tpi_id = tp_post
    AND tp_lang = language_of_user
    AND (NOT tp_deleted)
    AND (NOT tp_draft)
ORDER BY tpi_date DESC

$$;
