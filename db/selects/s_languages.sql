
CREATE OR REPLACE FUNCTION s_languages(
    language_of_user INT8
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

    CASE
        WHEN tln_name IS NULL
        THEN '[untranslated: ' || s_language_code_by_id(tl_id) || ']'
        ELSE tln_name
    END AS name,

    CASE
        WHEN tln_date IS NULL
        THEN ''
        ELSE tln_date::TEXT
    END AS date,

    c_language_has_all_names(tl_id) AS has_all_names
FROM t_languages
LEFT JOIN t_language_names
ON tl_id = tln_lang
    AND tln_name_lang = language_of_user
ORDER BY tln_name

$$;
