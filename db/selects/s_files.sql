
CREATE OR REPLACE FUNCTION s_files(
    results_number BIGINT,
    page_number BIGINT
)

RETURNS "FileDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- id
        tf_id,

        -- name
        tf_name,

        -- ext
        tf_ext,

        -- author
        tf_author,

        -- date
        tf_date::TEXT
    )::"FileDB"
    FROM t_files

    ORDER BY tf_date DESC

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
