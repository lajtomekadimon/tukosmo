
CREATE TYPE "WgiBlogPost" AS (
    req "WebsiteRequest",
    permalink TEXT
);

CREATE TYPE "WgoBlogPost" AS (
    data "WebsiteDataDB",
    routes "RouteDB"[],
    post "PostDB"
);


CREATE OR REPLACE FUNCTION ahw_g_blog_post(
    r "WgiBlogPost"
)

RETURNS "WgoBlogPost"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    post "PostDB";

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    post := s_post_by_lang_permalink(
        language_of_user,
        r.permalink
    );

    -- Check permalink
    IF post IS NULL THEN
        PERFORM err_wrong_post_permalink();
    END IF;

    -- Routes
    routes := s_post_routes_by_id_lang(
        post.id,
        language_of_user
    );

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- post
        post
    );

END;

$$;
