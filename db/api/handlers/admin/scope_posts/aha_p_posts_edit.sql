
CREATE TYPE "ApiPostsEdit" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT,
    title TEXT,
    description TEXT,
    body TEXT,
    permalink TEXT,
    draft BOOL,
    deleted BOOL,
    featured_image BIGINT,
    tags BIGINT[]
);


CREATE OR REPLACE FUNCTION aha_p_posts_edit(
    r "ApiPostsEdit"
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

    tag_id BIGINT;

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check post ID is correct
    IF NOT c_post_by_id(r.id) THEN
        PERFORM err_wrong_post_id();
    END IF;

    -- Check post title
    IF NOT e_is_title(r.title) THEN
        PERFORM err_wrong_title();
    END IF;

    -- Check post description
    IF NOT e_is_description(r.description) THEN
        PERFORM err_wrong_description();
    END IF;

    -- Check post body
    IF NOT e_is_body_text(r.body) THEN
        PERFORM err_wrong_body_text();
    END IF;

    -- Check post permalink
    IF NOT e_is_permalink(r.permalink) THEN
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

    -- Update existing post
    IF c_post_has_trans(
        r.id,
        (d.lang).id
    ) THEN

        PERFORM u_post(
            r.id,
            featured_image_id
        );

        PERFORM u_post_translation(
            r.id,
            (d.lang).id,
            r.title,
            r.description,
            r.body,
            r.permalink,
            r.draft,
            r.deleted
        );

    -- Create new translation of the post
    ELSE

        PERFORM u_post(
            r.id,
            featured_image_id
        );

        PERFORM i_post_translation(
            r.id,
            (d.lang).id,
            r.title,
            r.description,
            r.body,
            r.permalink,
            (d.userd).id,
            r.draft
        );

    END IF;

    PERFORM d_tags_of_post(r.id);

    FOREACH tag_id IN ARRAY r.tags LOOP
        -- Check tag ID is correct
        IF NOT c_tag_by_id(tag_id) THEN
            PERFORM err_wrong_tag_id();
        END IF;

        PERFORM i_tag_of_post(
            tag_id,
            r.id,
            (d.userd).id
        );
    END LOOP;

    -- Send post to trash if deleted=TRUE
    IF r.deleted THEN

        PERFORM u_post_to_trash(r.id);

    ELSE

        PERFORM u_post_out_of_trash(r.id);

    END IF;

END;

$$;
