
CREATE TYPE "ApiWebsite" AS (
    req "AdminRequest",
    csrf_token UUID,
    website_title TEXT,
    website_subtitle TEXT,
    copyright_owner TEXT,
    domain TEXT,
    default_lang TEXT
);


CREATE OR REPLACE FUNCTION aha_p_website(
    r "ApiWebsite"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check website's title in the new language
    IF NOT e_is_website_title(r.website_title) THEN
        PERFORM err_wrong_website_title();
    END IF;

    -- Check website's subtitle in the new language
    IF NOT e_is_website_subtitle(r.website_subtitle) THEN
        PERFORM err_wrong_website_subtitle();
    END IF;

    -- Check copyright owner in the new language
    IF NOT e_is_copyright_owner(r.website_subtitle) THEN
        PERFORM err_wrong_copyright_owner();
    END IF;

    -- TODO: Check domain

    -- Check default language
    IF NOT c_lang_by_code(r.default_lang) THEN
        PERFORM err_wrong_lang_code();
    END IF;

    PERFORM u_website_info(
        (d.lang).id,
        r.website_title,
        r.website_subtitle,
        r.copyright_owner
    );

END;

$$;
