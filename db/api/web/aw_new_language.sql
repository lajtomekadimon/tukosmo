
CREATE OR REPLACE FUNCTION aw_new_language(

    code_value TEXT,

    lang_ids BIGINT[],

    lang_names TEXT[]

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_code_id BIGINT;
    lang_name TEXT;
    lang_id BIGINT;

BEGIN

    lang_code_id := i_lang_code(code_value);

    FOR i IN 1..ARRAY_LENGTH(lang_names, 1) LOOP
        lang_id := lang_ids[i];
        lang_name := lang_names[i];

        PERFORM i_language(
            lang_code_id,
            lang_name,
            lang_id
        );
    END LOOP;

    RETURN lang_code_id;

END;

$$;
