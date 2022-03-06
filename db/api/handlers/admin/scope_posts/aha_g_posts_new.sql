
CREATE TYPE "AgiPostsNew" AS (
    req "AdminRequest",
    featured_image BIGINT,
    tags_added BIGINT[]
);

CREATE TYPE "AgoPostsNew" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    featured_image "FileDB",
    tags "TagDB"[],
    tags_added "TagDB"[]
);


CREATE OR REPLACE FUNCTION aha_g_posts_new(
    r "AgiPostsNew"
)

RETURNS "AgoPostsNew"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    featured_image "FileDB";

    tags "TagDB"[];

    tags_added "TagDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/posts/new',
        language_of_user
    );

    IF r.featured_image IS NOT NULL THEN
        featured_image := s_file_by_id(
            r.featured_image,
            language_of_user
        );

        -- Check file ID is correct
        IF featured_image IS NULL THEN
            PERFORM err_wrong_file_id();
        END IF;
    END IF;

    tags := s_tags(language_of_user);

    -- NOTE: Tag IDs are not cheched, but it's not necessary.
    tags_added := s_tags_by_ids(
        language_of_user,
        r.tags_added
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

        -- featured_image
        featured_image,

        -- tags
        tags,

        -- tags_added
        tags_added
    );

END;

$$;
