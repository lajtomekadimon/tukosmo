
CREATE OR REPLACE FUNCTION i_tag_translation(
    tag_id BIGINT,
    lang_id BIGINT,
    name_value TEXT,
    permalink_value TEXT,
    translator_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    this_tag_id BIGINT;

BEGIN

    INSERT INTO t_tag_translations (
        --ttt_id,
        ttt_tag,
        ttt_lang,
        ttt_name,
        ttt_permalink,
        ttt_translator
        --ttt_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        tag_id,
        lang_id,
        name_value,
        permalink_value,
        translator_id
        --NOW()
    ) RETURNING ttt_id INTO this_tag_id;

    RETURN this_tag_id;

END;

$$;

