
CREATE TYPE "DeleteLanguageARequest" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "DeleteLanguageAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    lang "LanguageDB"
);


CREATE OR REPLACE FUNCTION awa_delete_language(
    r "DeleteLanguageARequest"
)

RETURNS "DeleteLanguageAResponse"

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

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/delete_language?id=' || (r.id)::TEXT)::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    lang := s_language_by_id_lang(
        r.id,
        language_of_user
    );

    -- Check language ID is correct
    IF lang IS NULL THEN
        PERFORM err_wrong_lang_id();
    END IF;

    -- User is logged in
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
        lang
    );

END;

$$;
