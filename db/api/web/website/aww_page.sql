
CREATE TYPE "PageWRequest" AS (
    req "WebsiteRequest"
);

CREATE TYPE "PageWResponse" AS (
    data "WebsiteDataDB"
);


CREATE OR REPLACE FUNCTION aww_page(
    r "PageWRequest"
)

RETURNS "PageWResponse"

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

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request.';

    END IF;

    language_of_user := (d.lang).id;

    ---

    RETURN ROW(
        -- data
        d
    );

END;

$$;
