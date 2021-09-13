
CREATE OR REPLACE FUNCTION aw_admin_handler(

    session_id UUID,

    lang_id BIGINT

)

RETURNS TABLE(
    id BIGINT,
    email TEXT,
    name TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_user_by_session_lang(session_id, lang_id)

$$;
