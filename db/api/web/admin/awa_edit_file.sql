
CREATE TYPE "EditFileARequest" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "EditFileAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    file_data "FileDB"
);


CREATE OR REPLACE FUNCTION awa_edit_file(
    r "EditFileARequest"
)

RETURNS "EditFileAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    routes "RouteDB"[];

    language_of_user BIGINT;

    file_data "FileDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Routes
    routes := s_common_routes_by_route_lang(
        '/admin/edit_file?id=' || (r.id)::TEXT,
        language_of_user
    );

    file_data := s_file_by_id(r.id);

    -- Check file ID is correct
    IF file_data IS NULL THEN
        PERFORM err_wrong_file_id();
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

        -- file_data
        file_data
    );

END;

$$;
