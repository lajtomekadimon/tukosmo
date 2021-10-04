
CREATE TYPE "NewLanguageARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "NewLanguageAResponse" AS (
    data "AdminDataDB",
    languages "LanguageDB"[]
);


CREATE OR REPLACE FUNCTION awa_new_language(
    r "NewLanguageARequest"
)

RETURNS "NewLanguageAResponse"

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

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    language_of_user := (d.lang).id;

    langs := s_languages(language_of_user);

    -- TODO: Maybe I should consider empty array too?
    IF langs IS NULL THEN

        RAISE EXCEPTION 'TODO: Issue with languages.';

    END IF;


    RETURN (
        -- data
        d,

        -- languages
        langs
    )::"NewLanguageAResponse";

END;

$$;
