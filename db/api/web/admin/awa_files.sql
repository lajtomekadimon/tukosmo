
CREATE TYPE "FilesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "FilesAResponse" AS (
    data "AdminDataDB"
);


CREATE OR REPLACE FUNCTION awa_files(
    r "FilesARequest"
)

RETURNS "FilesAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    d := s_admin_handler_data(r.req);

    IF d IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    language_of_user := (d.lang).id;

    RETURN ROW(
        -- data
        d
    );

END;

$$;
