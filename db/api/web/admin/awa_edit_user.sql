
CREATE TYPE "EditUserARequest" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "EditUserAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    user_data "UserDB",
    i18n_names "NameDB"[]
);


CREATE OR REPLACE FUNCTION awa_edit_user(
    r "EditUserARequest"
)

RETURNS "EditUserAResponse"

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

    user_data "UserDB";

    i18n_names "NameDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Routes
    routes := ARRAY[]::"RouteDB"[];
    FOREACH langg IN ARRAY d.languages LOOP
        routes := ARRAY_APPEND(
            routes,
            (langg, '/admin/edit_user?id=' || (r.id)::TEXT)::"RouteDB"
        );
    END LOOP;

    language_of_user := (d.lang).id;

    user_data := s_user_by_id(r.id);

    -- Check user ID is correct
    IF user_data IS NULL THEN
        PERFORM err_wrong_user_id();
    END IF;

    i18n_names := s_user_names_by_id(
        r.id,
        language_of_user
    );

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

        -- user_data
        user_data,

        -- i18n_names
        i18n_names
    );

END;

$$;
