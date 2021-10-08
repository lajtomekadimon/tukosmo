
CREATE TYPE "PagesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "PagesAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_pages(
    r "PagesARequest"
)

RETURNS "PagesAResponse"

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
