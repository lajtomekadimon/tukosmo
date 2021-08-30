
CREATE OR REPLACE FUNCTION s_post_by_lang_permalink(
    language_of_user INT8,
    permalink_value TEXT
)

RETURNS TABLE(
    tp_id BIGINT,
    tp_title TEXT,
    tp_description TEXT,
    tp_body TEXT,
    tp_permalink TEXT,
    tp_author BIGINT,
    tp_author_name TEXT,
    tp_original_author BIGINT,
    tp_original_author_name TEXT,
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
    a.tu_name AS tp_author_name,
    tpi_author AS tp_original_author,
    b.tu_name AS tp_original_author_name,
    tpi_date AS tp_date,
    tp_date AS tp_date_trans
FROM t_post_ids

INNER JOIN t_posts
ON tpi_id = tp_post
    AND tp_lang = language_of_user
    AND tp_permalink = permalink_value
    AND (NOT tp_deleted)
    AND (NOT tp_draft)

INNER JOIN t_users a
ON tp_author = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC
LIMIT 1

$$;
