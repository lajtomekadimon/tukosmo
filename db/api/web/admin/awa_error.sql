
CREATE TYPE "ErrorARequest" AS (
    req "AdminRequest",
    code TEXT
);

CREATE TYPE "ErrorAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[]
);


CREATE OR REPLACE FUNCTION awa_error(
    r "ErrorARequest"
)

RETURNS "ErrorAResponse"

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
            (langg, '/admin/error?code=' || r.code)::"RouteDB"
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
