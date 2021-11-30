
CREATE TYPE "DeleteFileARequest" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "DeleteFileAResponse" AS (
    data "AdminDataDB",
    routes "RouteDB"[],
    csrf_token TEXT,
    file_data "FileDB"
);


CREATE OR REPLACE FUNCTION awa_delete_file(
    r "DeleteFileARequest"
)

RETURNS "DeleteFileAResponse"

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
        '/admin/delete_file?id=' || (r.id)::TEXT,
        language_of_user
    );

    file_data := s_file_by_id(
        r.id,
        language_of_user
    );

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
