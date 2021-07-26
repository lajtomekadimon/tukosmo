
CREATE OR REPLACE FUNCTION u_page(

    page_id BIGINT,

    title_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    author_id BIGINT,

    is_draft BOOL,

    is_deleted BOOL

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_pages
    SET tp_title = title_value,
        tp_body = body_value,
        tp_permalink = permalink_value,
        tp_draft = is_draft,
        tp_deleted = is_deleted
    WHERE tp_id = page_id;

    RETURN page_id;

END;

$$;
