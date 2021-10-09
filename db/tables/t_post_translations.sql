
CREATE TABLE t_post_translations (

    tpt_id          BIGSERIAL   PRIMARY KEY,
    tpt_post        BIGINT      NOT NULL REFERENCES t_posts,
    tpt_lang        BIGINT      NOT NULL REFERENCES t_languages,
    tpt_title       TEXT        NOT NULL CHECK(e_is_title(tpt_title)),
    tpt_description TEXT        NOT NULL CHECK(e_is_description(tpt_description)),
    tpt_body        TEXT        NOT NULL CHECK(e_is_body_text(tpt_body)),
    tpt_permalink   TEXT        NOT NULL CHECK(e_is_permalink(tpt_permalink)),
    tpt_translator  BIGINT      NOT NULL REFERENCES t_users,
    tpt_date        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tpt_draft       BOOL        NOT NULL,
    tpt_deleted     BOOL        NOT NULL,

    UNIQUE(tpt_post, tpt_lang),
    UNIQUE(tpt_lang, tpt_permalink)

);
