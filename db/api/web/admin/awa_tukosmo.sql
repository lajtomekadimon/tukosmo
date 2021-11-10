
CREATE TYPE "TukosmoARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "TukosmoAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[]
);


CREATE OR REPLACE FUNCTION awa_tukosmo(
    r "TukosmoARequest"
)

RETURNS "TukosmoAResponse"

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
            (langg, '/admin/tukosmo')::"RouteDB"
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
