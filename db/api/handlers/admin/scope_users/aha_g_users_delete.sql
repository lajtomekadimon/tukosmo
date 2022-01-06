
CREATE TYPE "AgiUsersDelete" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "AgoUsersDelete" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    user_data "UserDB"
);


CREATE OR REPLACE FUNCTION aha_g_users_delete(
    r "AgiUsersDelete"
)

RETURNS "AgoUsersDelete"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    user_data "UserDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/users/delete?id=' || (r.id)::TEXT,
        language_of_user
    );

    user_data := s_user_by_id_lang(
        r.id,
        language_of_user
    );

    -- Check user ID is correct
    IF user_data IS NULL THEN
        PERFORM err_wrong_user_id();
    END IF;

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT,

        -- user_data
        user_data
    );

END;

$$;
