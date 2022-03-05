
CREATE TYPE "AgiTagsEdit" AS (
    req "AdminRequest",
    tag BIGINT
);

CREATE TYPE "AgoTagsEdit" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    tag "TagDB",
    tag_translations "TagDB"[]
);


CREATE OR REPLACE FUNCTION aha_g_tags_edit(
    r "AgiTagsEdit"
)

RETURNS "AgoTagsEdit"

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

    tag_translations "TagDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/tags/edit?id=' || (r.tag)::TEXT,
        language_of_user
    );

    tag := s_tag_by_id(
        language_of_user,
        r.tag
    );

    -- Check tag ID is correct
    IF tag IS NULL THEN
        PERFORM err_wrong_tag_id();
    END IF;

    tag_translations := s_tag_translations_by_id(
        language_of_user,
        r.tag
    );

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
        tag,

        -- tag_translations
        tag_translations
    );

END;

$$;
