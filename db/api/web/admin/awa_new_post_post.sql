
CREATE TYPE "NewPostPostARequest" AS (
    req "AdminRequest",
    csrf_token UUID,
    post "PostDB",
    featured_image BIGINT
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

    language_of_user BIGINT;

    featured_image_id BIGINT;

    post_id BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
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

    -- Check featured image ID
    featured_image_id := NULL;
    IF r.featured_image IS NOT NULL THEN
        IF r.featured_image <> 0 THEN
            IF s_file_by_id(
                r.featured_image,
                language_of_user
            ) IS NULL THEN
                PERFORM err_wrong_file_id();
            ELSE
                featured_image_id := r.featured_image;
            END IF;
        END IF;
    END IF;

    post_id := i_post(
        featured_image_id,
        (d.userd).id
    );

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
