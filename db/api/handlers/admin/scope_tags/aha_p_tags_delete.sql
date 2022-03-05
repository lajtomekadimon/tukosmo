
CREATE TYPE "ApiTagsDelete" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT
);


CREATE OR REPLACE FUNCTION aha_p_tags_delete(
    r "ApiTagsDelete"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    tag_data "TagDB";

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

    -- Check tag ID is correct
    IF NOT c_tag_by_id(r.id) THEN
        PERFORM err_wrong_tag_id();
    END IF;

    PERFORM d_tag(r.id);

END;

$$;
