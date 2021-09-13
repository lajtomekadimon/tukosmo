
CREATE OR REPLACE FUNCTION s_current_language_by_code(
    lang_code TEXT
)

RETURNS TABLE(
    id INT8,
    code TEXT,
    name TEXT
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
        THEN '[untranslated: ' || tl_code || ']'
        ELSE tln_name
    END AS name

FROM t_languages
LEFT JOIN t_language_names
ON tl_id = tln_lang
    AND tln_name_lang = tl_id
WHERE tl_code = lang_code
LIMIT 1

$$;
