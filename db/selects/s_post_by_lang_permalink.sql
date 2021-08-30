
CREATE OR REPLACE FUNCTION s_post_by_lang_permalink(
    language_of_user INT8,
    permalink_value TEXT
)

RETURNS TABLE(
    tp_id BIGINT,
    tp_lang BIGINT,
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

    CASE
        WHEN tp_lang IS NULL
        THEN 0
        ELSE tp_lang
    END AS tp_lang,

    CASE
        WHEN tp_title IS NULL
        THEN ''
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

    tpi_date::TEXT AS tp_date,

    CASE
        WHEN tp_date IS NULL
        THEN ''
        ELSE tp_date::TEXT
    END AS tp_date_trans
FROM t_post_ids
LEFT JOIN t_posts
ON tpi_id = tp_post
    AND CASE
        WHEN tp_lang IS NULL
        THEN TRUE
        ELSE tp_lang = language_of_user
    END
    AND (NOT tp_deleted)
    AND (NOT tp_draft)
    AND tp_permalink = permalink_value
ORDER BY tpi_date DESC

$$;
