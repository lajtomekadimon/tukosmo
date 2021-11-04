
CREATE TYPE "PostsARequest" AS (
    req "AdminRequest",
    results_per_page BIGINT,
    page BIGINT,
    filter TEXT
);

CREATE TYPE "PostsAResponse" AS (
    data "AdminDataDB",
    posts "PostDB"[],
    filter TEXT,
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

    the_filter TEXT;

    total_results BIGINT;

    total_pages BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check the number of results per page is correct
    IF r.results_per_page < 1 THEN
        PERFORM err_wrong_rpp_number();
    END IF;

    IF r.filter = 'drafts' THEN

        posts := s_posts_f_drafts(
            language_of_user,
            r.results_per_page,
            r.page
        );

        the_filter := 'drafts';

        total_results := sc_posts_f_drafts(language_of_user);

    ELSIF r.filter = 'published' THEN

        posts := s_posts_f_published(
            language_of_user,
            r.results_per_page,
            r.page
        );

        the_filter := 'published';

        total_results := sc_posts_f_published(language_of_user);

    ELSIF r.filter = 'untranslated' THEN

        posts := s_posts_f_untranslated(
            language_of_user,
            r.results_per_page,
            r.page
        );

        the_filter := 'untranslated';

        total_results := sc_posts_f_untranslated(language_of_user);

    ELSIF r.filter = 'deleted' THEN

        posts := s_posts_f_deleted(
            language_of_user,
            r.results_per_page,
            r.page
        );

        the_filter := 'deleted';

        total_results := sc_posts_f_deleted(language_of_user);

    ELSE

        posts := s_posts(
            language_of_user,
            r.results_per_page,
            r.page
        );

        the_filter := 'all';

        total_results := sc_posts(language_of_user);

    END IF;

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

        -- posts
        posts,

        -- filter
        the_filter,

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
