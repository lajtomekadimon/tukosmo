
CREATE TYPE "LanguagesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "LanguagesAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    some_lang_without_names BOOL
);


CREATE OR REPLACE FUNCTION awa_languages(
    r "LanguagesARequest"
)

RETURNS "LanguagesAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];
    langg "LanguageDB";

    language_of_user BIGINT;

    some_lang_without_names BOOL := FALSE;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/languages')::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

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
