
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
    FROM t_languages
    LEFT JOIN t_language_names
    ON tl_id = tln_name_lang
        AND tln_lang = language_id
    WHERE tln_lang IS NULL
    LIMIT 1
)

$$;
