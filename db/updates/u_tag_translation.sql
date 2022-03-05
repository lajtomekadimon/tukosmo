
CREATE OR REPLACE FUNCTION u_tag_translation(
    tag_id BIGINT,
    lang_id BIGINT,
    name_value TEXT,
    permalink_value TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    tag_trans_id BIGINT;

BEGIN

    UPDATE t_tag_translations
    SET ttt_name = name_value,
        ttt_permalink = permalink_value
    WHERE ttt_tag = tag_id
        AND ttt_lang = lang_id
    RETURNING ttt_id INTO tag_trans_id;


    RETURN tag_trans_id;

END;

$$;
