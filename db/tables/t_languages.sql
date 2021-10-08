
CREATE TABLE t_languages (

    tl_id   BIGSERIAL   PRIMARY KEY,
    tl_code TEXT        NOT NULL UNIQUE CHECK(e_is_lang_code(tl_code)),
    tl_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
