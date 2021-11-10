
CREATE TYPE "ErrorWRequest" AS (
    req "WebsiteRequest",
    code TEXT
);

CREATE TYPE "ErrorWResponse" AS (
    data "WebsiteDataDB",
    routes "RouteDB"[]
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

    routes "RouteDB"[];
    langg "LanguageDB";

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/error?code=' || r.code)::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    ---

    RETURN ROW(
        -- data
        d,

        -- routes
        routes
    );

END;

$$;
