
CREATE TYPE "EditPostARequest" AS (
    req "AdminRequest",
    post BIGINT
);

CREATE TYPE "EditPostAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    post "PostDB",
    featured_image "FileDB"
);


CREATE OR REPLACE FUNCTION awa_edit_post(
    r "EditPostARequest"
)

RETURNS "EditPostAResponse"

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

    featured_image "FileDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/edit_post?id=' || (r.post)::TEXT,
        language_of_user
    );

    -- Check post ID is correct
    IF NOT c_post_by_id(r.post) THEN
        PERFORM err_wrong_post_id();
    END IF;

    post := s_post_by_id_lang(
        r.post,
        language_of_user
    );

    -- Currently, if NULL then it's a new translation
    /*IF post IS NULL THEN
        RAISE EXCEPTION '';
    END IF;*/

    featured_image := s_post_fimage_by_id_lang(
        r.post,
        language_of_user
    );

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
        post,

        -- featured_image
        featured_image
    );

END;

$$;
