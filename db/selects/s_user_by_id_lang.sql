
CREATE OR REPLACE FUNCTION s_user_by_id_lang(
    user_id BIGINT,
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
            user_id,
            lang_id
        ),
        tu_name
    ),
    -- suspended
    tu_suspended,
    -- date
    tu_date
)::"UserDB"

FROM t_users

WHERE tu_id = user_id

LIMIT 1

$$;
