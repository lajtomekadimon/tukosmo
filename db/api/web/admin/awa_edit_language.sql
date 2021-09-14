
CREATE OR REPLACE FUNCTION awa_edit_language(
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

SELECT s_language_names(
    language_of_user,
    language_id
)

$$;
