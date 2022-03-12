
CREATE TYPE "AgiPages" AS (
    req "AdminRequest",
    results_per_page BIGINT,
    page BIGINT
);

CREATE TYPE "AgoPages" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    results_per_page BIGINT,
    page BIGINT
);


CREATE OR REPLACE FUNCTION aha_g_pages(
    r "AgiPages"
)

RETURNS "AgoPages"

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
        '/admin/pages',
        language_of_user
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- results_per_page
        r.results_per_page,

        -- page
        r.page
    );

END;

$$;
