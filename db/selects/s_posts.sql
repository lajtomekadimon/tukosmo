
CREATE OR REPLACE FUNCTION s_posts(
    language_of_user BIGINT
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
    draft BOOL,
    deleted BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tpi_id AS id,

    COALESCE(tpt_id, 0) AS trans_id,

    COALESCE(tpt_lang, 0) AS lang,

    COALESCE(
        tpt_title,
        s_untrans_post_title_by_id(tpi_id)
    ) AS title,

    COALESCE(tpt_description, '') AS description,

    COALESCE(tpt_body, '') AS body,

    COALESCE(tpt_permalink, '') AS permalink,

    tpi_author AS author,

    b.tu_name AS author_name,

    COALESCE(tpt_translator, 0) AS translator,

    COALESCE(a.tu_name, '') AS translator_name,

    tpi_date::TEXT AS date,

    COALESCE(tpt_date::TEXT, '') AS date_trans,

    COALESCE(tpt_draft, TRUE) AS draft,

    COALESCE(tpt_deleted, FALSE) AS deleted
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
