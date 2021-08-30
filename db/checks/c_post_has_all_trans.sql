
CREATE OR REPLACE FUNCTION c_post_has_all_trans(

    post_id BIGINT

)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT NOT EXISTS(
    SELECT 1
    FROM t_languages
    LEFT JOIN t_posts
    ON tl_id = tp_lang
        AND tp_post = post_id
    WHERE tp_lang IS NULL
    LIMIT 1
)

$$;
