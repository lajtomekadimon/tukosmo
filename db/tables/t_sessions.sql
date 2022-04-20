
CREATE TABLE t_sessions (

    ts_id         UUID        PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),
    ts_user       BIGINT      NOT NULL
                              REFERENCES t_users ON DELETE CASCADE,
    ts_ip         TEXT        NOT NULL,
    ts_browser    TEXT        NOT NULL,
    ts_platform   TEXT        NOT NULL,
    ts_csrf_token UUID        NOT NULL UNIQUE DEFAULT GEN_RANDOM_UUID(),
    ts_date       TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
