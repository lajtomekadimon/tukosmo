
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_code_id BIGINT;

BEGIN

    PERFORM i_user(
        'test@test.com',
        CRYPT('12345', GEN_SALT('bf')),
        'Tukosmo'
    );

    lang_code_id := i_lang_code('en');

    PERFORM i_language(
        lang_code_id,
        'English',
        lang_code_id
    );

END;

$$;
