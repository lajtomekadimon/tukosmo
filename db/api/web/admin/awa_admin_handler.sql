
CREATE OR REPLACE FUNCTION awa_admin_handler(

    session_id UUID,

    lang_code TEXT

)

RETURNS "AdminDataDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_admin_handler_data(
    (
        session_id,
        lang_code
    )::"AdminRequest"
)

$$;
