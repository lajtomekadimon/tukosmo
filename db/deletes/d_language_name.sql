
CREATE OR REPLACE FUNCTION d_language_name(

    lang_name_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_language_names
    WHERE tln_id = lang_name_id;

    RETURN lang_name_id;

END;

$$;
