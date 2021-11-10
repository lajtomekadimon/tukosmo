
CREATE TYPE "LoginARequest" AS (
    req "WebsiteRequest"
);

CREATE TYPE "LoginAResponse" AS (
    data "WebsiteDataDB",
    routes "RouteDB"[]
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

    routes "RouteDB"[];

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/login',
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
