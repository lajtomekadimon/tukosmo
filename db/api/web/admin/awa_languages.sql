
CREATE OR REPLACE FUNCTION awa_languages(
    language_of_user BIGINT
)

RETURNS "LanguageDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_languages(language_of_user)

$$;
