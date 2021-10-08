
CREATE TYPE "LanguagesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "LanguagesAResponse" AS (
    data "AdminDataDB",
    languages "LanguageDB"[]
);


CREATE OR REPLACE FUNCTION awa_languages(
    r "LanguagesARequest"
)

RETURNS "LanguagesAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    langs "LanguageDB"[];

BEGIN

    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    langs := s_languages(language_of_user);

    -- TODO: Maybe I should consider empty array too?
    IF langs IS NULL THEN

        RAISE EXCEPTION 'TODO: There''s something wrong with languages.';

    END IF;

    RETURN ROW(
        -- data
        d,

        -- languages
        langs
    );

END;

$$;
