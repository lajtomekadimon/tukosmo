
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
    tp_translator BIGINT,
    tp_translator_name TEXT,
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
    tpt_title AS tp_title,
    tpt_description AS tp_description,
    tpt_body AS tp_body,
    tpt_permalink AS tp_permalink,
    tpi_author AS tp_author,
    b.tu_name AS tp_author_name,
    tpt_translator AS tp_translator,
    a.tu_name AS tp_translator_name,
    tpi_date AS tp_date,
    tpt_date AS tp_date_trans
FROM t_post_ids

INNER JOIN t_post_translations
ON tpi_id = tpt_post
    AND tpt_lang = language_of_user
    AND tpt_permalink = permalink_value
    AND (NOT tpt_deleted)
    AND (NOT tpt_draft)

INNER JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC
LIMIT 1

$$;
