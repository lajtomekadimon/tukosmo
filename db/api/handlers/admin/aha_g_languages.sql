
CREATE TYPE "AgiLanguages" AS (
    req "AdminRequest"
);

CREATE TYPE "AgoLanguages" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    some_lang_without_names BOOL
);


CREATE OR REPLACE FUNCTION aha_g_languages(
    r "AgiLanguages"
)

RETURNS "AgoLanguages"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    some_lang_without_names BOOL := FALSE;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/languages',
        language_of_user
    );

    some_lang_without_names := EXISTS(
        SELECT 1
        FROM UNNEST(d.languages) lang
        WHERE NOT lang.has_all_names
        LIMIT 1
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- some_lang_without_names
        some_lang_without_names
    );

END;

$$;
