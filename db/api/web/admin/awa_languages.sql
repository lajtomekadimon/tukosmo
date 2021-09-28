
CREATE TYPE "LanguagesARequest" AS (
    req "AdminRequest"
);

CREATE TYPE "LanguagesAResponse" AS (
    data "AdminDataDB",
    languages "LanguageDB"[]
);


CREATE OR REPLACE FUNCTION awa_languages(
    r "LanguagesARequest"
)

RETURNS "LanguagesAResponse"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

WITH variables (handler_data, language_of_user) AS (
    SELECT d, (d.lang).id
    FROM s_admin_handler_data(r.req) d
)

SELECT (
    -- data
    handler_data,

    -- languages
    s_languages(language_of_user)
)::"LanguagesAResponse"

FROM variables

$$;
