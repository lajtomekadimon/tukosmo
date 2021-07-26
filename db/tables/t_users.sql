
CREATE TABLE t_users (

    tu_id       BIGSERIAL PRIMARY KEY,
    tu_email    TEXT      NOT NULL UNIQUE CHECK (e_is_email(tu_email)),
    tu_password TEXT      NOT NULL,
    /* bcrypt has a limit, so when inserting the password it must be checked */
    /* Insert: CRYPT('__the_password__', GEN_SALT('bf'))
     * Check: tu_password = CRYPT('__the_password__', tu_password)
     */

    tu_date TIMESTAMPTZ NOT NULL DEFAULT NOW()

);
