
CREATE OR REPLACE FUNCTION s_user_by_session_lang(
    session_id UUID,
    lang_id BIGINT
)

RETURNS "UserDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- id
    tu_id,
    -- email
    tu_email,
    -- name
    COALESCE(
        s_user_name_by_user_lang(
            tu_id,
            lang_id
        ),
        tu_name
    ),
    -- admin
    tu_admin,
    -- suspended
    tu_suspended,
    -- date
    tu_date
)::"UserDB"

FROM t_users
INNER JOIN t_sessions
ON ts_user = tu_id
    AND ts_id = session_id
LIMIT 1

$$;
