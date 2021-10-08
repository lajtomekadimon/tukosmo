
CREATE TYPE "PostsARequest" AS (
    req "AdminRequest",
    results_per_page BIGINT,
    page BIGINT
);

CREATE TYPE "PostsAResponse" AS (
    data "AdminDataDB",
    posts "PostDB"[],
    results_per_page BIGINT,
    page BIGINT,
    total_results BIGINT,
    total_pages BIGINT
);


CREATE OR REPLACE FUNCTION awa_posts(
    r "PostsARequest"
)

RETURNS "PostsAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    posts "PostDB"[];

    total_results BIGINT;

    total_pages BIGINT;

BEGIN

    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    posts := s_posts(
        language_of_user,
        r.results_per_page,
        r.page
    );

    -- TODO: Maybe I should consider empty array too?
    IF posts IS NULL THEN

        RAISE EXCEPTION 'TODO: There''s something wrong with posts.';

    END IF;

    total_results := sc_posts(language_of_user);

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
