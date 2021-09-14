
CREATE OR REPLACE FUNCTION s_current_language_by_code(
    lang_code TEXT
)

RETURNS TABLE(
    id INT8,
    code TEXT,
    name TEXT,
    date TEXT,
    has_all_names BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tl_id AS id,

    tl_code AS code,

    COALESCE(
        tln_name,
        '[untranslated: ' || tl_code || ']'
    ) AS name,

    COALESCE(tln_date::TEXT, '') AS date,

    c_language_has_all_names(tl_id) AS has_all_names
FROM t_languages
LEFT JOIN t_language_names
ON tl_id = tln_lang
    AND tln_name_lang = tl_id
WHERE tl_code = lang_code
LIMIT 1

$$;
