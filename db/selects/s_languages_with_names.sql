
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
    a.tl_lang_code AS tl_id,
    a.tl_name AS tl_name,
    b.tl_name AS tl_trans_name
FROM t_languages a
LEFT JOIN t_languages b
ON a.tl_lang_code = b.tl_lang
WHERE a.tl_lang = language_of_user
    AND b.tl_lang_code = language_id

$$;
