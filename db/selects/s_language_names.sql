
CREATE OR REPLACE FUNCTION s_language_names(
    language_of_user INT8,
    language_id INT8
)

RETURNS TABLE(
    name TEXT,
    lang_id INT8,
    lang_code TEXT,
    lang_name TEXT,
    lang_date TEXT,
    lang_has_all_names BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    COALESCE(
        s_name_of_language(
            language_id,
            tl_id
        ),
        ''
    ) AS name,

    tl_id AS lang_id,

    tl_code AS lang_code,

    COALESCE(
        tln_name,
        '[untranslated: ' || tl_code || ']'
    ) AS name,

    COALESCE(tln_date::TEXT, '') AS date,

    c_language_has_all_names(tl_id) AS lang_has_all_names
FROM t_languages
LEFT JOIN t_language_names
ON tl_id = tln_lang
    AND tln_name_lang = language_of_user
ORDER BY tln_name

$$;
