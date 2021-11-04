
CREATE TYPE "NewUserARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "NewUserAResponse" AS (
    data "AdminDataDB",
    csrf_token TEXT
);


CREATE OR REPLACE FUNCTION awa_new_user(
    r "NewUserARequest"
)

RETURNS "NewUserAResponse"

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
