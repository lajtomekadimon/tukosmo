
CREATE TYPE "ApiJsonEditFile" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT,
    file_title TEXT
);


CREATE OR REPLACE FUNCTION aha_p_json_edit_file(
    r "ApiJsonEditFile"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    file_data "FileDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    file_data := s_file_by_id(
        r.id,
        language_of_user
    );

    -- Check file ID is correct
    IF file_data IS NULL THEN
        PERFORM err_wrong_file_id();
    END IF;

    -- Check file title
    IF NOT e_is_file_title(r.file_title) THEN
        PERFORM err_wrong_file_title();
    END IF;

    -- Check file title is unique
    IF c_file_by_title(r.file_title) AND file_data.title <> r.file_title THEN
        PERFORM err_file_title_already_exists();
    END IF;

    PERFORM u_file(
        r.id,
        r.file_title
    );

END;

$$;
