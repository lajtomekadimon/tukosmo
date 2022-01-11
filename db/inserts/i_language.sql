
CREATE OR REPLACE FUNCTION i_language(
    code_value TEXT,
    title_value TEXT,
    subtitle_value TEXT,
    copyright_owner_value TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_id BIGINT;

BEGIN

    INSERT INTO t_languages (
        --tl_id,
        tl_code,
        --tl_date,
        tl_website_title,
        tl_website_subtitle,
        tl_copyright_owner
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        code_value,
        --NOW(),
        title_value,
        subtitle_value,
        copyright_owner_value
    ) RETURNING tl_id INTO lang_id;

    RETURN lang_id;

END;

$$;
