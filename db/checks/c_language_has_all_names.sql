
CREATE OR REPLACE FUNCTION c_language_has_all_names(

    language_id BIGINT

)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT NOT EXISTS(
    SELECT 1
    FROM t_lang_codes
    LEFT JOIN t_languages
    ON tlc_id = tl_lang
        AND tl_lang_code = language_id
    WHERE tl_lang IS NULL
    LIMIT 1
)

$$;
