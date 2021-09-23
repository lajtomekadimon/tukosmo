
CREATE OR REPLACE FUNCTION awa_edit_language(
    language_of_user INT8,
    language_id INT8
)

RETURNS "NameDB"[]

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
