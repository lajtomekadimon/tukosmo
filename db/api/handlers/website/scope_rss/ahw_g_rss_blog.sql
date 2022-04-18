
CREATE TYPE "WgiRssBlog" AS (
    req "WebsiteRequest",
    http_data "HTTPDataDB",
    results BIGINT
);

CREATE TYPE "WgoRssBlog" AS (
    data "WebsiteDataDB",
    routes "RouteDB"[],
    posts "PostDB"[]
);


CREATE OR REPLACE FUNCTION ahw_g_rss_blog(
    r "WgiRssBlog"
)

RETURNS "WgoRssBlog"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    posts "PostDB"[];

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/rss/blog',
        language_of_user
    );

    -- Check the number of results per page is correct
    IF r.results < 1 THEN
        PERFORM err_wrong_rpp_number();
    END IF;

    posts := s_posts_by_pref_lang(
        language_of_user,
        r.results,
        1  -- page
    );

    /* VISITS STATS
     ***************/
    PERFORM i_stats_visit(
        (d.lang).code,
        '/rss/blog',
        (r.http_data).ip,
        (r.http_data).referrer,
        (r.http_data).browser,
        (r.http_data).platform
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- posts
        posts
    );

END;

$$;
