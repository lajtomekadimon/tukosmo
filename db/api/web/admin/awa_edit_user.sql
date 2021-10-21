
CREATE TYPE "EditUserARequest" AS (
    req "AdminRequest",
    id BIGINT
);

CREATE TYPE "EditUserAResponse" AS (
    data "AdminDataDB",
    user_data "UserDB"
);


CREATE OR REPLACE FUNCTION awa_edit_user(
    r "EditUserARequest"
)

RETURNS "EditUserAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    user_data "UserDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- TODO: Check user ID

    user_data := s_user_by_id_lang(
        r.id,
        language_of_user
    );

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- user_data
        user_data
    );

END;

$$;
