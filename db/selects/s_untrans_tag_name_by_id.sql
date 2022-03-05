
CREATE OR REPLACE FUNCTION s_untrans_tag_name_by_id(
    tag_id BIGINT
)

RETURNS TEXT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ttt_name
FROM t_tags

INNER JOIN t_tag_translations
ON tt_id = ttt_tag
    AND ttt_tag = tag_id

ORDER BY CASE
    WHEN s_language_id_by_code('en') IS NULL
    THEN 2
    ELSE CASE
        WHEN ttt_lang = s_language_id_by_code('en')
        THEN 1
        ELSE 2
    END
END

LIMIT 1

$$;
