
CREATE TYPE "UsersARequest" AS (
    req "AdminRequest",
    results_per_page BIGINT,
    page BIGINT
);

CREATE TYPE "UsersAResponse" AS (
    data "AdminDataDB",
    users "UserDB"[],
    results_per_page BIGINT,
    page BIGINT,
    total_results BIGINT,
    total_pages BIGINT
);


CREATE OR REPLACE FUNCTION awa_users(
    r "UsersARequest"
)

RETURNS "UsersAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    users "UserDB"[];

    total_results BIGINT;

    total_pages BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    users := s_users(
        language_of_user,
        r.results_per_page,
        r.page
    );

    total_results := sc_users(language_of_user);

    total_pages := CEIL(total_results / r.results_per_page::NUMERIC);

    RETURN ROW(
        -- data
        d,

        -- users
        users,

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
