
CREATE TYPE "NewPostARequest" AS (
    req "AdminRequest",
    featured_image BIGINT
);

CREATE TYPE "NewPostAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    featured_image "FileDB"
);


CREATE OR REPLACE FUNCTION awa_new_post(
    r "NewPostARequest"
)

RETURNS "NewPostAResponse"

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

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/new_post',
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
        featured_image
    );

END;

$$;
