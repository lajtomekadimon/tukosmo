
CREATE OR REPLACE FUNCTION s_lname_id_by_lang_nlang(

    lang_id BIGINT,

    name_lang_id BIGINT

)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tln_id
FROM t_language_names
WHERE tln_lang = lang_id
    AND tln_name_lang = name_lang_id
LIMIT 1

$$;
