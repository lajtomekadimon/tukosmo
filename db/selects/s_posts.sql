
CREATE OR REPLACE FUNCTION s_posts(
    language_of_user INT8
)

RETURNS TABLE(
    id BIGINT,
    trans_id BIGINT,
    lang BIGINT,
    title TEXT,
    description TEXT,
    body TEXT,
    permalink TEXT,
    author BIGINT,
    author_name TEXT,
    translator BIGINT,
    translator_name TEXT,
    date TEXT,
    date_trans TEXT,
    has_all_trans BOOL,
    draft BOOL,
    untranslated BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tpi_id AS id,

    CASE
        WHEN tpt_id IS NULL
        THEN 0
        ELSE tpt_id
    END AS trans_id,

    CASE
        WHEN tpt_lang IS NULL
        THEN 0
        ELSE tpt_lang
    END AS lang,

    CASE
        WHEN tpt_title IS NULL
        THEN s_untrans_post_title_by_id(tpi_id)
        ELSE tpt_title::TEXT
    END AS title,

    CASE
        WHEN tpt_description IS NULL
        THEN ''
        ELSE tpt_description::TEXT
    END AS description,

    CASE
        WHEN tpt_body IS NULL
        THEN ''
        ELSE tpt_body::TEXT
    END AS body,

    CASE
        WHEN tpt_permalink IS NULL
        THEN ''
        ELSE tpt_permalink::TEXT
    END AS permalink,

    tpi_author AS author,

    b.tu_name AS author_name,

    CASE
        WHEN tpt_translator IS NULL
        THEN 0
        ELSE tpt_translator
    END AS translator,

    CASE
        WHEN a.tu_name IS NULL
        THEN ''
        ELSE a.tu_name
    END AS translator_name,

    tpi_date::TEXT AS date,

    CASE
        WHEN tpt_date IS NULL
        THEN ''
        ELSE tpt_date::TEXT
    END AS date_trans,

    c_post_has_all_trans(tpi_id) AS has_all_trans,

    CASE
        WHEN tpt_draft IS NULL
        THEN TRUE
        ELSE tpt_draft
    END AS draft,

    CASE
        WHEN tpt_lang IS NULL
        THEN TRUE
        ELSE FALSE
    END AS untranslated
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
