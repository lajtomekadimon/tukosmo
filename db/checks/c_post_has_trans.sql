
CREATE OR REPLACE FUNCTION c_post_has_trans(
    post_id BIGINT,
    lang_id BIGINT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM t_post_translations
    WHERE tpt_post = post_id
        AND tpt_lang = lang_id
    LIMIT 1
)

$$;
