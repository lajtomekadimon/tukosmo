
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_en BIGINT;
    lang_es BIGINT;

    user_id BIGINT;

    file_id BIGINT;

    post_id BIGINT;
    post_title TEXT;
    post_description TEXT;
    permalink_value TEXT;
    post_body TEXT;

BEGIN

    user_id := 1;

    file_id := i_file(
        'featured-image-default-post.jpg',
        1943885,
        user_id
    );

    PERFORM u_file(
        file_id,
        'Ocean With Rock Formation Under Starry Night'
    );

    post_id := i_post(
        file_id,
        user_id
    );

    post_title := 'Example post';
    post_description :=
        'An example post.';
    permalink_value := 'example-post';
    post_body := '<p>Hello world!</p>';

    PERFORM i_post_translation(
        post_id,
        s_language_id_by_code('en'),
        post_title,
        post_description,
        post_body,
        permalink_value,
        1,  -- user ID
        FALSE
    );

END;

$$;
