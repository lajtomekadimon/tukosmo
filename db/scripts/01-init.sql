
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_en BIGINT;
    lang_es BIGINT;

BEGIN

    PERFORM i_user(
        'test@test.com',
        CRYPT('12345', GEN_SALT('bf')),
        'Tukosmo'
    );

    lang_en := i_language('en');

    PERFORM i_language_name(
        lang_en,
        'English',
        lang_en
    );

    lang_es := i_language('es');

    PERFORM i_language_name(
        lang_es,
        'Spanish',
        lang_en
    );

    PERFORM i_language_name(
        lang_es,
        'Español',
        lang_es
    );

    PERFORM i_language_name(
        lang_en,
        'Inglés',
        lang_es
    );

END;

$$;
