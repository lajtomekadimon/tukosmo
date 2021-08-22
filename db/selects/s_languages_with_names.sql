
CREATE OR REPLACE FUNCTION s_languages_with_names(
    language_of_user INT8,
    language_id INT8
)

RETURNS TABLE(
    tl_id INT8,
    tl_name TEXT,
    tl_trans_name TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT
    tlc_id AS tl_id,
    a.tl_name AS tl_name,
    CASE
        WHEN s_name_of_language(
            language_id,
            tlc_id
        ) IS NULL THEN ''
        ELSE s_name_of_language(
            language_id,
            tlc_id
        )
    END AS tl_trans_name
FROM t_lang_codes
LEFT JOIN t_languages a
ON tlc_id = a.tl_lang_code
WHERE a.tl_lang = language_of_user
ORDER BY tl_name

$$;
