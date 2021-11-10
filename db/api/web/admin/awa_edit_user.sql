
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

    language_of_user BIGINT;

    user_data "UserDB";

    i18n_names "NameDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/edit_user?id=' || (r.id)::TEXT,
        language_of_user
    );

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
