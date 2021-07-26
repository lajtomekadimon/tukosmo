
CREATE OR REPLACE FUNCTION u_language(

    language_id BIGINT,

    name_value TEXT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_languages
    SET tl_name = name_value
    WHERE tl_id = language_id;

    RETURN language_id;

END;

$$;
