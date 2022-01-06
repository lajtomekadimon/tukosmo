
CREATE TYPE "WgiBlog" AS (
    req "WebsiteRequest",
    results_per_page BIGINT,
    page BIGINT
);

CREATE TYPE "WgoBlog" AS (
    data "WebsiteDataDB",
    routes "RouteDB"[],
    posts "PostDB"[],
    results_per_page BIGINT,
    page BIGINT,
    total_results BIGINT,
    total_pages BIGINT
);


CREATE OR REPLACE FUNCTION ahw_g_blog(
    r "WgiBlog"
)

RETURNS "WgoBlog"

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

    total_results BIGINT;

    total_pages BIGINT;

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        -- TODO: Hide p and rpp when first page and default rpp
        '/blog?p=' || (r.page)::TEXT
            || '&rpp=' || (r.results_per_page)::TEXT,
        language_of_user
    );

    -- Check the number of results per page is correct
    IF r.results_per_page < 1 THEN
        PERFORM err_wrong_rpp_number();
    END IF;

    posts := s_posts_by_pref_lang(
        language_of_user,
        r.results_per_page,
        r.page
    );

    total_results := sc_posts_by_pref_lang(language_of_user);

    total_pages := CASE total_results
        WHEN 0 THEN 1
        ELSE CEIL(total_results / r.results_per_page::NUMERIC)
    END;

    -- Check the number page is correct
    IF (r.page < 1) OR (r.page > total_pages) THEN
        PERFORM err_wrong_page_number();
    END IF;

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- posts
        posts,

        -- results_per_page
        r.results_per_page,

        -- page
        r.page,

        -- total_results
        total_results,

        -- total_pages
        total_pages
    );

END;

$$;
