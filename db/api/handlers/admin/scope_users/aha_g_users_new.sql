
CREATE TYPE "AgiUsersNew" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoUsersNew" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT
);


CREATE OR REPLACE FUNCTION aha_g_users_new(
    r "AgiUsersNew"
)

RETURNS "AgoUsersNew"

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
        '/admin/users/new',
        language_of_user
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT
    );

END;

$$;
