
CREATE OR REPLACE FUNCTION s_languages(
    language_of_user INT8
)

RETURNS TABLE(
    tl_id INT8,
    tl_code TEXT,
    tl_name TEXT,
    tl_date TEXT,
    tl_has_all_names BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tl_id AS tl_id,
    tl_code AS tl_code,
    tln_name AS tl_name,
    tln_date::TEXT AS tl_date,
    c_language_has_all_names(tl_id) AS tl_has_all_names
FROM t_languages
LEFT JOIN t_language_names
ON tl_id = tln_lang
WHERE tln_name_lang = language_of_user
ORDER BY tln_name

$$;
