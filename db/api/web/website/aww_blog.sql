
CREATE TYPE "BlogWRequest" AS (
    req "WebsiteRequest"
    --results_per_page BIGINT,
    --page BIGINT
);

CREATE TYPE "BlogWResponse" AS (
    data "WebsiteDataDB",
    posts "PostDB"[],
    --results_per_page BIGINT,
    --page BIGINT,
    total_results BIGINT
    --total_pages BIGINT
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

BEGIN

    d := s_website_handler_data(r.req);

    language_of_user := (d.lang).id;

    posts := s_posts_by_lang(language_of_user);

    total_results := sc_posts_by_lang(language_of_user);

    RETURN ROW(
        -- data
        d,

        -- posts
        posts,

        -- total_results
        total_results
    );

END;

$$;
