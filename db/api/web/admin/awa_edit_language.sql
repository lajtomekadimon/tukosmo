
CREATE TYPE "EditLanguageARequest" AS (
    req "AdminRequest",
    lang BIGINT
);

CREATE TYPE "EditLanguageAResponse" AS (
    data "AdminDataDB",
    lang "LanguageDB",
    names "NameDB"[]
);


CREATE OR REPLACE FUNCTION awa_edit_language(
    r "EditLanguageARequest"
)

RETURNS "EditLanguageAResponse"

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

    -- lang
    s_language_by_id_lang(
        r.lang,
        language_of_user
    ),

    -- names
    s_language_names(
        r.lang,
        language_of_user
    )
)::"EditLanguageAResponse"

FROM variables

$$;
