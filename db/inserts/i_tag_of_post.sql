
CREATE OR REPLACE FUNCTION i_tag_of_post(
    tag_id BIGINT,
    post_id BIGINT,
    author_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    this_tot_id BIGINT;

BEGIN

    INSERT INTO t_tags_of_posts (
        --ttop_id,
        ttop_tag,
        ttop_post,
        ttop_author
        --ttop_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        tag_id,
        post_id,
        author_id
        --NOW()
    ) RETURNING ttop_id INTO this_tot_id;

    RETURN this_tot_id;

END;

$$;

