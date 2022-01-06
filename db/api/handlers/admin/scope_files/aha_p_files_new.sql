
CREATE TYPE "ApiFilesNew" AS (
    req "AdminRequest",
    --csrf_token UUID,
    filename TEXT
);


CREATE OR REPLACE FUNCTION aha_p_files_new(
    r "ApiFilesNew"
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    file_id BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Check CSRF token
    /*
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;
    */

    -- Check file name
    IF NOT e_is_filename(r.filename) THEN
        PERFORM err_wrong_filename();
    END IF;

    -- Check file name is unique
    IF c_file_by_name(r.filename) THEN
        PERFORM err_filename_already_exists();
    END IF;

    file_id := i_file(
        r.filename,
        (d.userd).id
    );

    RETURN file_id;

END;

$$;
