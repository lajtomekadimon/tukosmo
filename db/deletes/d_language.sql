
CREATE OR REPLACE FUNCTION d_language(

    language_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_languages
    WHERE tl_id = language_id;

    RETURN language_id;

END;

$$;
