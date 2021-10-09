
CREATE TYPE "EditPostPostARequest" AS (
    req "AdminRequest",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION awa_edit_post_post(
    r "EditPostPostARequest"
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

    -- Check post title
    IF NOT e_is_title((r.post).title) THEN
        PERFORM err_wrong_title;
    END IF;

    -- TODO: Check post description

    -- TODO: Check post body

    -- TODO: Check post permalink

    -- Update existing post
    IF c_post_has_trans(
        (r.post).id,
        (d.lang).id
    ) THEN

        -- TODO: Check post ID

        PERFORM u_post_translation(
            (r.post).id,
            (d.lang).id,
            (r.post).title,
            (r.post).description,
            (r.post).body,
            (r.post).permalink,
            (r.post).draft,
            (r.post).deleted
        );

    -- Create new translation of the post
    ELSE

        PERFORM i_post_translation(
            (r.post).id,
            (d.lang).id,
            (r.post).title,
            (r.post).description,
            (r.post).body,
            (r.post).permalink,
            (d.userd).id,
            (r.post).draft
        );

    END IF;

END;

$$;
