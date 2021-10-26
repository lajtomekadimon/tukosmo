
CREATE TYPE "DeleteLanguagePostARequest" AS (
    req "AdminRequest",
    id BIGINT
);


CREATE OR REPLACE FUNCTION awa_delete_language_post(
    r "DeleteLanguagePostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check language ID is correct
    IF s_language_by_id_lang(
        r.id,
        language_of_user
    ) IS NULL THEN
        PERFORM err_wrong_lang_id();
    END IF;

    PERFORM d_language(r.id);

END;

$$;
