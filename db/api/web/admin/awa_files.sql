
CREATE TYPE "FilesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "FilesAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[]
);


CREATE OR REPLACE FUNCTION awa_files(
    r "FilesARequest"
)

RETURNS "FilesAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/files',
        language_of_user
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes
    );

END;

$$;
