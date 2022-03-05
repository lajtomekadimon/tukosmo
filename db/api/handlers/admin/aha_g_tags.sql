
CREATE TYPE "AgiTags" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoTags" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    tags "TagDB"[]
);


CREATE OR REPLACE FUNCTION aha_g_tags(
    r "AgiTags"
)

RETURNS "AgoTags"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    tags "TagDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/tags',
        language_of_user
    );

    tags := s_tags(
        language_of_user
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- tags
        tags
    );

END;

$$;
