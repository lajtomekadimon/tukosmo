
CREATE TABLE t_user_names (

    tun_id BIGSERIAL PRIMARY KEY,
    tun_user BIGINT NOT NULL REFERENCES t_users,
    tun_name TEXT NOT NULL CHECK (e_is_user_name(tun_name)),
    tun_lang BIGINT NOT NULL REFERENCES t_languages,

    tun_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tun_user, tun_lang)

);
