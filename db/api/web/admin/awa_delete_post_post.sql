
CREATE TYPE "DeletePostPostARequest" AS (
    req "AdminRequest",
    id BIGINT
);


CREATE OR REPLACE FUNCTION awa_delete_post_post(
    r "DeletePostPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    -- TODO: Check post ID is correct

    -- TODO: Set deleted to TRUE, and after another request, delete
    PERFORM d_post(r.id);

END;

$$;
