
CREATE TYPE "LoginARequest" AS (
    req "WebsiteRequest"
);

CREATE TYPE "LoginAResponse" AS (
    data "WebsiteDataDB"
);


CREATE OR REPLACE FUNCTION awa_login(
    r "LoginARequest"
)

RETURNS "LoginAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    language_of_user BIGINT;

BEGIN

    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d
    );

END;

$$;
