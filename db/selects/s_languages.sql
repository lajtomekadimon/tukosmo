
CREATE OR REPLACE FUNCTION s_languages(
    language_of_user INT8
)

RETURNS TABLE(
    tlc_id INT8,
    tlc_code TEXT,
    tl_name TEXT,
    tl_date TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tlc_id, tlc_code, tl_name, tl_date::TEXT
FROM t_lang_codes
LEFT JOIN t_languages
ON tlc_id = tl_lang_code
WHERE tl_lang = language_of_user
ORDER BY tl_name

$$;
