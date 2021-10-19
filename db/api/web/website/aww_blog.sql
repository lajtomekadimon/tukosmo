
CREATE TYPE "BlogWRequest" AS (
    req "WebsiteRequest",
    results_per_page BIGINT,
    page BIGINT
);

CREATE TYPE "BlogWResponse" AS (
    data "WebsiteDataDB",
    posts "PostDB"[],
    results_per_page BIGINT,
    page BIGINT,
    total_results BIGINT,
    total_pages BIGINT
);


CREATE OR REPLACE FUNCTION aww_blog(
    r "BlogWRequest"
)

RETURNS "BlogWResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "WebsiteDataDB";

    language_of_user BIGINT;

    posts "PostDB"[];

    total_results BIGINT;

    total_pages BIGINT;

BEGIN

    -- Check request and select common data
    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    posts := s_posts_by_pref_lang(
        language_of_user,
        r.results_per_page,
        r.page
    );

    total_results := sc_posts_by_pref_lang(language_of_user);

    total_pages := CEIL(total_results / r.results_per_page::NUMERIC);

    RETURN ROW(
        -- data
        d,

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
