
CREATE TYPE "NewPostPostARequest" AS (
    req "AdminRequest",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION awa_new_post_post(
    r "NewPostPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    post_id BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Check post title
    IF NOT e_is_title((r.post).title) THEN
        PERFORM err_wrong_title;
    END IF;

    -- Check post description
    IF NOT e_is_description((r.post).description) THEN
        PERFORM err_wrong_description;
    END IF;

    -- Check post body
    IF NOT e_is_body_text((r.post).body) THEN
        PERFORM err_wrong_body_text;
    END IF;

    -- TODO: Check post permalink

    IF (r.post).id = 0 THEN

        post_id := i_post((d.userd).id);

    ELSE

        post_id := (r.post).id;

        -- TODO: Check post ID

    END IF;

    PERFORM i_post_translation(
        post_id,
        (d.lang).id,
        (r.post).title,
        (r.post).description,
        (r.post).body,
        (r.post).permalink,
        (d.userd).id,
        (r.post).draft
    );

END;

$$;
