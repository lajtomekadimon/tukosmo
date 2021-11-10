
CREATE TYPE "SessionsARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "SessionsAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    sessions "SessionDB"[]
);


CREATE OR REPLACE FUNCTION awa_sessions(
    r "SessionsARequest"
)

RETURNS "SessionsAResponse"

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

    sessions "SessionDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/sessions')::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    sessions := s_sessions_by_user((d.userd).id);

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT,

        -- sessions
        sessions
    );

END;

$$;

