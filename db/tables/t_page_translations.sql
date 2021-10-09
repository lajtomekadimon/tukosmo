
CREATE TABLE t_page_translations (

    tpt_id         BIGSERIAL   PRIMARY KEY,
    tpt_page       BIGINT      NOT NULL REFERENCES t_pages,
    tpt_lang       BIGINT      NOT NULL REFERENCES t_languages,
    tpt_title      TEXT        NOT NULL CHECK(e_is_title(tpt_title)),
    tpt_body       TEXT        NOT NULL,
    tpt_permalink  TEXT        NOT NULL,  /* TODO: Check max length */
    tpt_translator BIGINT      NOT NULL REFERENCES t_users,
    tpt_date       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tpt_draft      BOOL        NOT NULL,
    tpt_deleted    BOOL        NOT NULL,

    UNIQUE(tpt_page, tpt_lang),
    UNIQUE(tpt_lang, tpt_permalink)

);
