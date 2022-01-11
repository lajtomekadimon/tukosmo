
CREATE TYPE "AgiWebsite" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoWebsite" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    website_title TEXT,
    website_subtitle TEXT,
    copyright_owner TEXT
);


CREATE OR REPLACE FUNCTION aha_g_website(
    r "AgiWebsite"
)

RETURNS "AgoWebsite"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    website_title_value TEXT;
    website_subtitle_value TEXT;
    copyright_owner_value TEXT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/website',
        language_of_user
    );

    website_title_value := s_website_title_by_lang(language_of_user);
    website_subtitle_value := s_website_subtitle_by_lang(language_of_user);
    copyright_owner_value := s_copyright_owner_by_lang(language_of_user);

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT,

        -- website_title
        website_title_value,

        -- website_subtitle
        website_subtitle_value,
        
        -- copyright_owner
        copyright_owner_value
    );

END;

$$;
