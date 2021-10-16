
CREATE TYPE "ErrorWRequest" AS (
    req "WebsiteRequest"
);

CREATE TYPE "ErrorWResponse" AS (
    data "WebsiteDataDB"
);


CREATE OR REPLACE FUNCTION aww_error(
    r "ErrorWRequest"
)

RETURNS "ErrorWResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    ---

    RETURN ROW(
        -- data
        d
    );

END;

$$;
