
CREATE TYPE "EditPostPostARequest" AS (
    req "AdminRequest",
    csrf_token UUID,
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

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check post ID is correct
    IF NOT c_post_by_id((r.post).id) THEN
        PERFORM err_wrong_post_id();
    END IF;

    -- Check post title
    IF NOT e_is_title((r.post).title) THEN
        PERFORM err_wrong_title();
    END IF;

    -- Check post description
    IF NOT e_is_description((r.post).description) THEN
        PERFORM err_wrong_description();
    END IF;

    -- Check post body
    IF NOT e_is_body_text((r.post).body) THEN
        PERFORM err_wrong_body_text();
    END IF;

    -- Check post permalink
    IF NOT e_is_permalink((r.post).permalink) THEN
        PERFORM err_wrong_permalink();
    END IF;

    -- Update existing post
    IF c_post_has_trans(
        (r.post).id,
        (d.lang).id
    ) THEN

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

    -- Send post to trash if deleted=TRUE
    IF (r.post).deleted THEN

        PERFORM u_post_to_trash((r.post).id);

    ELSE

        PERFORM u_post_out_of_trash((r.post).id);

    END IF;

END;

$$;
