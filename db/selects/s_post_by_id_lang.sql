
CREATE OR REPLACE FUNCTION s_post_by_id_lang(
    post_id BIGINT,
    post_lang BIGINT
)

RETURNS TABLE(
    tp_id BIGINT,
    tp_title TEXT,
    tp_description TEXT,
    tp_body TEXT,
    tp_permalink TEXT,
    tp_author BIGINT,
    tp_author_name TEXT,
    tp_translator BIGINT,
    tp_translator_name TEXT,
    tp_date TEXT,
    tp_date_trans TEXT,
    tp_draft BOOL,
    tp_deleted BOOL
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
    tpi_author AS tp_author,
    b.tu_name AS tp_author_name,
    tp_translator AS tp_translator,
    a.tu_name AS tp_translator_name,
    tpi_date AS tp_date,
    tp_date AS tp_date_trans,
    tp_draft AS tp_draft,
    tp_deleted AS tp_deleted
FROM t_post_ids

INNER JOIN t_posts
ON tpi_id = tp_post
    AND tp_lang = post_lang
    AND tpi_id = post_id

INNER JOIN t_users a
ON tp_translator = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC
LIMIT 1

$$;
