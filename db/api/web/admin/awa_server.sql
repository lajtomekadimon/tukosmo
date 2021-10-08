
CREATE TYPE "ServerARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "ServerAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_server(
    r "ServerARequest"
)

RETURNS "ServerAResponse"

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
