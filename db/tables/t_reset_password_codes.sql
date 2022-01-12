
CREATE TABLE t_reset_password_codes (

    trpc_id            UUID        PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),
    trpc_user          BIGINT      NOT NULL
                                   REFERENCES t_users ON DELETE CASCADE,
    trpc_date            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    trpc_expiration_date TIMESTAMPTZ NOT NULL

);
