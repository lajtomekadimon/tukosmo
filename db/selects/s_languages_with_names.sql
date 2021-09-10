
CREATE OR REPLACE FUNCTION s_languages_with_names(
    language_of_user INT8,
    language_id INT8
)

RETURNS TABLE(
    id INT8,
    name TEXT,
    trans_name TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tl_id AS id,

    CASE
        WHEN a.tln_name IS NULL
        THEN '[untranslated: ' || s_language_code_by_id(tl_id) || ']'
        ELSE a.tln_name
    END AS name,
        
    CASE
        WHEN s_name_of_language(
            language_id,
            tl_id
        ) IS NULL
        THEN ''
        ELSE s_name_of_language(
            language_id,
            tl_id
        )
    END AS trans_name
FROM t_languages
LEFT JOIN t_language_names a
ON tl_id = a.tln_lang
    AND a.tln_name_lang = language_of_user
ORDER BY a.tln_name

$$;
