
CREATE TYPE "AgiTagsDelete" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "AgoTagsDelete" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    tag "TagDB"
);


CREATE OR REPLACE FUNCTION aha_g_tags_delete(
    r "AgiTagsDelete"
)

RETURNS "AgoTagsDelete"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    tag "TagDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/tags/delete?id=' || (r.id)::TEXT,
        language_of_user
    );

    tag := s_tag_by_id(
        language_of_user,
        r.id
    );

    -- Check tag ID is correct
    IF tag IS NULL THEN
        PERFORM err_wrong_tag_id();
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

        -- tag
        tag
    );

END;

$$;
