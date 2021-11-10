
CREATE TYPE "NewPostARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "NewPostAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT
);


CREATE OR REPLACE FUNCTION awa_new_post(
    r "NewPostARequest"
)

RETURNS "NewPostAResponse"

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

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/new_post')::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d,

        -- routes
        routes,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT
    );

END;

$$;
