
DO

LANGUAGE PLPGSQL

$$

DECLARE

    user_id BIGINT;

    lang_en BIGINT;
    lang_es BIGINT;

BEGIN

    user_id := i_user(
        'test@test.com',
        CRYPT('12345', GEN_SALT('bf')),
        'Admin'
    );

    PERFORM u_user_by_admin(
        user_id,
        'test@test.com',
        'Admin',
        TRUE,  -- admin
        FALSE  -- suspended
    );

    lang_en := i_language(
        'en',
        'Example',
        'Website built with Tukosmo',
        'Example'
    );

    PERFORM i_language_name(
        lang_en,
        'English',
        lang_en
    );

    lang_es := i_language(
        'es',
        'Ejemplo',
        'Página web construida con Tukosmo',
        'Ejemplo'
    );

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
