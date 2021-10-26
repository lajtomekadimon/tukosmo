
CREATE TYPE "DeletePostARequest" AS (
    req "AdminRequest",
    post BIGINT
);

CREATE TYPE "DeletePostAResponse" AS (
    data "AdminDataDB",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION awa_delete_post(
    r "DeletePostARequest"
)

RETURNS "DeletePostAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    post "PostDB";

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    post := s_post_by_id_lang(
        r.post,
        language_of_user
    );

    IF post IS NULL THEN
        PERFORM err_wrong_post_id();
    END IF;

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- post
        post
    );

END;

$$;
