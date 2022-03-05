
CREATE TABLE t_tags_of_posts (

    ttop_id     BIGSERIAL   PRIMARY KEY,
    ttop_tag    BIGINT      NOT NULL
                            REFERENCES t_tags ON DELETE CASCADE,
    ttop_post   BIGINT      NOT NULL
                            REFERENCES t_posts ON DELETE CASCADE,
    ttop_author BIGINT      NOT NULL
                            REFERENCES t_users ON DELETE CASCADE,
    ttop_date   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(ttop_tag, ttop_post)

);

