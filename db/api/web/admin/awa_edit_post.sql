
CREATE TYPE "EditPostARequest" AS (
    req "AdminRequest",
    post BIGINT
);

CREATE TYPE "EditPostAResponse" AS (
    data "AdminDataDB",
    post "PostDB"
);


CREATE OR REPLACE FUNCTION awa_edit_post(
    r "EditPostARequest"
)

RETURNS "EditPostAResponse"

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

    -- post
    s_post_by_id_lang(
        r.post,
        language_of_user
    )
)::"EditPostAResponse"

FROM variables

$$;
