
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_code_en BIGINT;
    lang_code_es BIGINT;

BEGIN

    PERFORM i_user(
        'test@test.com',
        CRYPT('12345', GEN_SALT('bf')),
        'Tukosmo'
    );

    lang_code_en := i_lang_code('en');

    PERFORM i_language(
        lang_code_en,
        'English',
        lang_code_en
    );

    lang_code_es := i_lang_code('es');

    PERFORM i_language(
        lang_code_es,
        'Spanish',
        lang_code_en
    );

    PERFORM i_language(
        lang_code_es,
        'Español',
        lang_code_es
    );

    PERFORM i_language(
        lang_code_en,
        'Inglés',
        lang_code_es
    );

END;

$$;
