
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
    tpt_title AS tp_title,
    tpt_description AS tp_description,
    tpt_body AS tp_body,
    tpt_permalink AS tp_permalink,
    tpi_author AS tp_author,
    b.tu_name AS tp_author_name,
    tpt_translator AS tp_translator,
    a.tu_name AS tp_translator_name,
    tpi_date AS tp_date,
    tpt_date AS tp_date_trans,
    tpt_draft AS tp_draft,
    tpt_deleted AS tp_deleted
FROM t_post_ids

INNER JOIN t_post_translations
ON tpi_id = tpt_post
    AND tpt_lang = post_lang
    AND tpi_id = post_id

INNER JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC
LIMIT 1

$$;
