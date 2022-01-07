
CREATE TYPE "ApiJsonEditFile" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT,
    filename TEXT
);


CREATE OR REPLACE FUNCTION aha_p_json_edit_file(
    r "ApiJsonEditFile"
)

RETURNS TEXT

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    file_data "FileDB";

    ofilename TEXT;

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

    ofilename := file_data.name;

    -- Check file name
    IF NOT e_is_filename(r.filename) THEN
        PERFORM err_wrong_filename();
    END IF;

    -- Check file name is unique
    IF c_file_by_name(r.filename) THEN
        PERFORM err_filename_already_exists();
    END IF;

    PERFORM u_file(
        r.id,
        r.filename
    );

    RETURN ofilename;

END;

$$;
