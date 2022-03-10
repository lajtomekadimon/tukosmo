
CREATE OR REPLACE FUNCTION s_users(
    language_of_user BIGINT,
    results_number BIGINT,
    page_number BIGINT
)

RETURNS "UserDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- id
        tu_id,

        -- email
        tu_email,

        -- name
        COALESCE(
            s_user_name_by_user_lang(
                tu_id,
                language_of_user
            ),
            tu_name
        ),

        -- suspended
        tu_suspended,

        -- date
        tu_date::TEXT
    )::"UserDB"
    FROM t_users

    ORDER BY tu_date DESC

    /* IMPORTANT NOTE:
     * Pagination using LIMIT + OFFSET is not an efficient solution, as it gets
     * slower and slower as the table grows. Indexing immutable rows could be
     * a solution, but then you'd have pages with different number of results
     * once you delete something.
     */
    LIMIT results_number
    OFFSET (page_number - 1) * results_number
)

$$;
