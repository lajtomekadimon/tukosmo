
CREATE OR REPLACE FUNCTION aw_edit_language_post(

    language_id BIGINT,

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

    lang_name TEXT;
    lang_id BIGINT;

    lang_name_id BIGINT;

BEGIN

    PERFORM u_language(
        language_id,
        code_value
    );

    FOR i IN 1..ARRAY_LENGTH(lang_names, 1) LOOP
        lang_id := lang_ids[i];
        lang_name := lang_names[i];

        lang_name_id = s_lname_id_by_lang_nlang(
            language_id,
            lang_id
        );

        IF lang_name_id IS NULL THEN

            PERFORM i_language_name(
                language_id,
                lang_name,
                lang_id
            );

        ELSE

            PERFORM u_language_name(
                lang_name_id,
                lang_name
            );

        END IF;
    END LOOP;

    RETURN language_id;

END;

$$;
