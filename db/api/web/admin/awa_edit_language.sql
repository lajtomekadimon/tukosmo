
CREATE TYPE "EditLanguageARequest" AS (
    req "AdminRequest",
    lang BIGINT
);

CREATE TYPE "EditLanguageAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    lang "LanguageDB",
    names "NameDB"[]
);


CREATE OR REPLACE FUNCTION awa_edit_language(
    r "EditLanguageARequest"
)

RETURNS "EditLanguageAResponse"

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

    lang "LanguageDB";

    language_names "NameDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/edit_language?id=' || (r.lang)::TEXT)::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    lang := s_language_by_id_lang(
        r.lang,
        language_of_user
    );

    -- Check language ID is correct
    IF lang IS NULL THEN
        PERFORM err_wrong_lang_id();
    END IF;

    language_names := s_language_names(
        r.lang,
        language_of_user
    );

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT,

        -- lang
        lang,

        -- names
        language_names
    );

END;

$$;
