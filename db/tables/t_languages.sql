
CREATE TABLE t_languages (

    tl_id   BIGSERIAL   PRIMARY KEY,
    tl_code TEXT        NOT NULL UNIQUE CHECK(e_is_lang_code(tl_code)),
    tl_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    tl_website_title    TEXT NOT NULL
                             CHECK(e_is_website_title(tl_website_title)),
    tl_website_subtitle TEXT NOT NULL
                             CHECK(e_is_website_subtitle(tl_website_subtitle)),
    tl_copyright_owner  TEXT NOT NULL
                             CHECK(e_is_copyright_owner(tl_copyright_owner))

);
