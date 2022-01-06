
CREATE TYPE "AgiPostsEdit" AS (
    req "AdminRequest",
    post BIGINT,
    featured_image BIGINT,
    first_request BOOL
);

CREATE TYPE "AgoPostsEdit" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    post "PostDB",
    featured_image "FileDB"
);


CREATE OR REPLACE FUNCTION aha_g_posts_edit(
    r "AgiPostsEdit"
)

RETURNS "AgoPostsEdit"

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
        '/admin/posts/edit?id=' || (r.post)::TEXT,
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

    IF r.first_request THEN
        featured_image := s_post_fimage_by_id_lang(
            r.post,
            language_of_user
        );
    ELSIF r.featured_image IS NOT NULL THEN
        featured_image := s_file_by_id(
            r.featured_image,
            language_of_user
        );

        -- Check file ID is correct
        IF featured_image IS NULL THEN
            PERFORM err_wrong_file_id();
        END IF;
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
        post,

        -- featured_image
        featured_image
    );

END;

$$;
