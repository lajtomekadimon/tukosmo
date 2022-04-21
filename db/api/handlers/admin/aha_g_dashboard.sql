
CREATE TYPE "AgiDashboard" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoDashboard" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    visitors_last_month BIGINT
);


CREATE OR REPLACE FUNCTION aha_g_dashboard(
    r "AgiDashboard"
)

RETURNS "AgoDashboard"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    visitors_last_month BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin',
        language_of_user
    );

    visitors_last_month := sc_stats_visitors_last_month();

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- visitors_last_month
        visitors_last_month
    );

END;

$$;
