
CREATE TYPE "ApiPostsNew" AS (
    req "AdminRequest",
    csrf_token UUID,
    post "PostDB",
    featured_image BIGINT,
    tags BIGINT[]
);


CREATE OR REPLACE FUNCTION aha_p_posts_new(
    r "ApiPostsNew"
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

    file_data "FileDB";

    featured_image_id BIGINT;

    post_id BIGINT;

    tag_id BIGINT;

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
            file_data := s_file_by_id(
                r.featured_image,
                language_of_user
            );

            IF file_data IS NULL THEN
                PERFORM err_wrong_file_id();
            ELSE
                IF NOT e_ext_is_image(file_data.ext) THEN
                    PERFORM err_featured_image_is_not_image();
                END IF;
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

    FOREACH tag_id IN ARRAY r.tags LOOP
        -- Check tag ID is correct
        IF NOT c_tag_by_id(tag_id) THEN
            PERFORM err_wrong_tag_id();
        END IF;

        PERFORM i_tag_of_post(
            tag_id,
            post_id,
            (d.userd).id
        );
    END LOOP;

END;

$$;
