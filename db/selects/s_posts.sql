
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
    tp_translator BIGINT,
    tp_translator_name TEXT,
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
        WHEN tpt_id IS NULL
        THEN 0
        ELSE tpt_id
    END AS tp_trans_id,

    CASE
        WHEN tpt_lang IS NULL
        THEN 0
        ELSE tpt_lang
    END AS tp_lang,

    CASE
        WHEN tpt_title IS NULL
        THEN s_untrans_post_title_by_id(tpi_id)
        ELSE tpt_title::TEXT
    END AS tp_title,

    CASE
        WHEN tpt_description IS NULL
        THEN ''
        ELSE tpt_description::TEXT
    END AS tp_description,

    CASE
        WHEN tpt_body IS NULL
        THEN ''
        ELSE tpt_body::TEXT
    END AS tp_body,

    CASE
        WHEN tpt_permalink IS NULL
        THEN ''
        ELSE tpt_permalink::TEXT
    END AS tp_permalink,

    tpi_author AS tp_author,

    b.tu_name AS tp_author_name,

    CASE
        WHEN tpt_translator IS NULL
        THEN 0
        ELSE tpt_translator
    END AS tp_translator,

    CASE
        WHEN a.tu_name IS NULL
        THEN ''
        ELSE a.tu_name
    END AS tp_translator_name,

    tpi_date::TEXT AS tp_date,

    CASE
        WHEN tpt_date IS NULL
        THEN ''
        ELSE tpt_date::TEXT
    END AS tp_date_trans,

    c_post_has_all_trans(tpi_id) AS tp_has_all_trans,

    CASE
        WHEN tpt_draft IS NULL
        THEN TRUE
        ELSE tpt_draft
    END AS tp_draft,

    CASE
        WHEN tpt_lang IS NULL
        THEN TRUE
        ELSE FALSE
    END AS tp_untranslated
FROM t_post_ids

LEFT JOIN t_post_translations
ON tpi_id = tpt_post
    AND CASE
        WHEN tpt_lang IS NULL
        THEN TRUE
        ELSE tpt_lang = language_of_user
    END
    AND NOT tpt_deleted

LEFT JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tpi_author = b.tu_id

ORDER BY tpi_date DESC

$$;
