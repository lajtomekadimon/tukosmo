
CREATE TYPE "WebsiteARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "WebsiteAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_website(
    r "WebsiteARequest"
)

RETURNS "WebsiteAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d
    );

END;

$$;
