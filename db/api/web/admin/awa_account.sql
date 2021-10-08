
CREATE TYPE "AccountARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "AccountAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_account(
    r "AccountARequest"
)

RETURNS "AccountAResponse"

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
        d
    );

END;

$$;
