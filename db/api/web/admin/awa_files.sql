
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
    langg "LanguageDB";

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/files')::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d,

        -- routes
        routes
    );

END;

$$;
