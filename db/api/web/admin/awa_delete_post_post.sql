
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

    language_of_user BIGINT;

    post "PostDB";

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    post := s_post_by_id_lang(
        r.id,
        language_of_user
    );

    -- Check post ID is correct
    IF post IS NULL THEN
        PERFORM err_wrong_post_id();
    END IF;

    IF post.deleted THEN

        PERFORM d_post(r.id);

    ELSE

        PERFORM u_post_to_trash(r.id);

    END IF;

END;

$$;
