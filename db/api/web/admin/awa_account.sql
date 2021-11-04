
CREATE TYPE "AccountARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "AccountAResponse" AS (
    data "AdminDataDB",
    csrf_token TEXT,
    user_data "UserDB",
    i18n_names "NameDB"[]
);


CREATE OR REPLACE FUNCTION awa_account(
    r "AccountARequest"
)

RETURNS "AccountAResponse"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

    user_data "UserDB";

    i18n_names "NameDB"[];

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    user_data := s_user_by_id((d.userd).id);

    i18n_names := s_user_names_by_id(
        (d.userd).id,
        language_of_user
    );

    -- User is logged in
    RETURN ROW(
        -- data
        d,

        -- csrf_token
        s_csrf_token_by_session(
            (r.req).session
        )::TEXT,

        -- user_data
        user_data,

        -- i18n_names
        i18n_names
    );

END;

$$;
