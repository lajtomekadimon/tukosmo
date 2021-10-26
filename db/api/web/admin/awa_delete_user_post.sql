
CREATE TYPE "DeleteUserPostARequest" AS (
    req "AdminRequest",
    id BIGINT
);


CREATE OR REPLACE FUNCTION awa_delete_user_post(
    r "DeleteUserPostARequest"
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

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- TODO: Check user ID is correct

    PERFORM d_user(r.id);

END;

$$;
