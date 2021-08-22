
CREATE OR REPLACE FUNCTION s_name_of_language(
    language_id INT8,
    language_of_name INT8
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tl_name
FROM t_languages
WHERE tl_lang = language_of_name
    AND tl_lang_code = language_id

$$;
