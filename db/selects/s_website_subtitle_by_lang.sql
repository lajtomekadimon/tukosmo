
CREATE OR REPLACE FUNCTION s_website_subtitle_by_lang(
    language_of_user BIGINT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tl_website_subtitle
FROM t_languages
WHERE tl_id = language_of_user
LIMIT 1

$$;
