
CREATE TYPE "NewLanguageARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "NewLanguageAResponse" AS (
    data "AdminDataDB",
    csrf_token TEXT
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

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT
    );

END;

$$;
