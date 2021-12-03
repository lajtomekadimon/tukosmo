
CREATE TYPE "DeleteFilePostARequest" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT
);


CREATE OR REPLACE FUNCTION awa_delete_file_post(
    r "DeleteFilePostARequest"
)

RETURNS TEXT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    file_data "FileDB";

BEGIN

    -- Check request
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

    PERFORM u_posts_del_images(r.id);

    PERFORM d_file(r.id);

    RETURN file_data.name;

END;

$$;
