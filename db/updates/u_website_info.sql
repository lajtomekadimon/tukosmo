
CREATE OR REPLACE FUNCTION u_website_info(
    lang_id BIGINT,
    website_title_value TEXT,
    website_subtitle_value TEXT,
    copyright_owner_value TEXT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_languages
    SET tl_website_title = website_title_value,
        tl_website_subtitle = website_subtitle_value,
        tl_copyright_owner = copyright_owner_value
    WHERE tl_id = lang_id;

END;

$$;
