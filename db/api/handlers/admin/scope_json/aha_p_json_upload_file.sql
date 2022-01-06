
CREATE TYPE "ApiJsonUploadFile" AS (
    req "AdminRequest"
);

CREATE TYPE "ApoJsonUploadFile" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION aha_p_json_upload_file(
    r "ApiJsonUploadFile"
)

RETURNS "ApoJsonUploadFile"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- User is logged in
    RETURN ROW(
        -- data
        d
    );

END;

$$;
