
CREATE TYPE "EditLanguageARequest" AS (
    req "AdminRequest",
    lang BIGINT
);

CREATE TYPE "EditLanguageAResponse" AS (
    data "AdminDataDB",
    lang "LanguageDB",
    names "NameDB"[]
);


CREATE OR REPLACE FUNCTION awa_edit_language(
    r "EditLanguageARequest"
)

RETURNS "EditLanguageAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    lang "LanguageDB";

    language_names "NameDB"[];

BEGIN

    d := s_admin_handler_data(r.req);

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    language_of_user := (d.lang).id;

    lang := s_language_by_id_lang(
        r.lang,
        language_of_user
    );

    IF lang IS NULL THEN

        RAISE EXCEPTION 'TODO: Language ID is not correct.';

    END IF;

    language_names := s_language_names(
        r.lang,
        language_of_user
    );

    -- TODO: Maybe I should consider empty array too?
    IF language_names IS NULL THEN

        RAISE EXCEPTION 'TODO: There''s something wrong with language names.';

    END IF;


    RETURN ROW(
        -- data
        d,

        -- lang
        lang,

        -- names
        language_names
    );

END;

$$;
