
CREATE TABLE t_post_translations (

    tpt_id          BIGSERIAL   PRIMARY KEY,
    tpt_post        BIGINT      NOT NULL REFERENCES t_post_ids,
    tpt_lang        BIGINT      NOT NULL REFERENCES t_languages,
    tpt_title       TEXT        NOT NULL,  /* TODO: Check max length */
    tpt_description TEXT        NOT NULL,  /* TODO: Check max length */
    tpt_body        TEXT        NOT NULL,
    tpt_permalink   TEXT        NOT NULL,  /* TODO: Check max length */
    tpt_translator  BIGINT      NOT NULL REFERENCES t_users,
    tpt_date        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tpt_draft       BOOL        NOT NULL,
    tpt_deleted     BOOL        NOT NULL,

    UNIQUE(tpt_post, tpt_lang),
    UNIQUE(tpt_lang, tpt_permalink)

);
