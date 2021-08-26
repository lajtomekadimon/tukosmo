
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

SELECT tln_name
FROM t_language_names
WHERE tln_name_lang = language_of_name
    AND tln_lang = language_id

$$;
