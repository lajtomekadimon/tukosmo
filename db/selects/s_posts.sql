
CREATE OR REPLACE FUNCTION s_posts(
    language_of_user INT8
)

RETURNS TABLE(
    tp_id BIGINT,
    tp_trans_id BIGINT,
    tp_lang BIGINT,
    tp_title TEXT,
    tp_description TEXT,
    tp_body TEXT,
    tp_permalink TEXT,
    tp_author BIGINT,
    tp_author_name TEXT,
    tp_original_author BIGINT,
    tp_original_author_name TEXT,
    tp_date TEXT,
    tp_date_trans TEXT,
    tp_has_all_trans BOOL,
    tp_draft BOOL,
    tp_untranslated BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tpi_id AS tp_id,

    CASE
        WHEN tp_id IS NULL
        THEN 0
        ELSE tp_id
    END AS tp_trans_id,

    CASE
        WHEN tp_lang IS NULL
        THEN 0
        ELSE tp_lang
    END AS tp_lang,

    CASE
        WHEN tp_title IS NULL
        THEN s_untrans_post_title_by_id(tpi_id)
        ELSE tp_title::TEXT
    END AS tp_title,

    CASE
        WHEN tp_description IS NULL
        THEN ''
        ELSE tp_description::TEXT
    END AS tp_description,

    CASE
        WHEN tp_body IS NULL
        THEN ''
        ELSE tp_body::TEXT
    END AS tp_body,

    CASE
        WHEN tp_permalink IS NULL
        THEN ''
        ELSE tp_permalink::TEXT
    END AS tp_permalink,

    CASE
        WHEN tp_author IS NULL
        THEN 0
        ELSE tp_author
    END AS tp_author,

    CASE
        WHEN a.tu_name IS NULL
        THEN ''
        ELSE a.tu_name
    END AS tp_author_name,

    tpi_author AS tp_original_author,

    b.tu_name AS tp_original_author_name,

    tpi_date::TEXT AS tp_date,

    CASE
        WHEN tp_date IS NULL
        THEN ''
        ELSE tp_date::TEXT
    END AS tp_date_trans,

    c_post_has_all_trans(tpi_id) AS tp_has_all_trans,

    CASE
        WHEN tp_draft IS NULL
        THEN TRUE
        ELSE tp_draft
    END AS tp_draft,

    CASE
        WHEN tp_lang IS NULL
        THEN TRUE
        ELSE FALSE
    END AS tp_untranslated
FROM t_post_ids

LEFT JOIN t_posts
ON tpi_id = tp_post
    AND CASE
        WHEN tp_lang IS NULL
        THEN TRUE
        ELSE tp_lang = language_of_user
    END
    AND NOT tp_deleted

LEFT JOIN t_users a
ON tp_author = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC

$$;
