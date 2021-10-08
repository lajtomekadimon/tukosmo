
CREATE TYPE "EditPostARequest" AS (
    req "AdminRequest",
    post BIGINT
);

CREATE TYPE "EditPostAResponse" AS (
    data "AdminDataDB",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION awa_edit_post(
    r "EditPostARequest"
)

RETURNS "EditPostAResponse"

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

    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    post := s_post_by_id_lang(
        r.post,
        language_of_user
    );

    -- Currently, if NULL then it's a new translation
    /*IF post IS NULL THEN

        RAISE EXCEPTION 'TODO: Post ID is not correct.';

    END IF;*/

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- post
        post
    );

END;

$$;
