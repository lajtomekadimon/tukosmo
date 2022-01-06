
CREATE TYPE "AgiPostsDelete" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "AgoPostsDelete" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    post "PostDB"
);


CREATE OR REPLACE FUNCTION aha_g_posts_delete(
    r "AgiPostsDelete"
)

RETURNS "AgoPostsDelete"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    post "PostDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/posts/delete?id=' || (r.id)::TEXT,
        language_of_user
    );

    post := s_post_by_id_lang(
        r.id,
        language_of_user
    );

    -- Check post ID is correct
    IF post IS NULL THEN
        PERFORM err_wrong_post_id();
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

        -- post
        post
    );

END;

$$;
