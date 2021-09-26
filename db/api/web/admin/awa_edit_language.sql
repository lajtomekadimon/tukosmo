
CREATE TYPE "EditLanguageAR" AS (
    req "AdminRequest",
    lang BIGINT
);

CREATE TYPE "EditLanguageAH" AS (
    data "AdminDataDB",
    lang "LanguageDB",
    names "NameDB"[]
);


CREATE OR REPLACE FUNCTION query_db(
    r "EditLanguageAR"
)

RETURNS "EditLanguageAH"

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
)::"EditLanguageAH"

FROM variables

$$;
