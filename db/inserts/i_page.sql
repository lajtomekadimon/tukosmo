
CREATE OR REPLACE FUNCTION i_page(

    page_id BIGINT,

    lang_id BIGINT,

    title_value TEXT,

    body_value TEXT,

    permalink_value TEXT,

    author_id BIGINT,

    is_draft BOOL

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    this_page_id BIGINT;

BEGIN

    INSERT INTO t_pages (
        --tp_id,
        tp_page,
        tp_lang,
        tp_title,
        tp_body,
        tp_permalink,
        tp_author,
        --tp_date,
        tp_draft,
        tp_deleted
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        page_id,
        lang_id,
        title_value,
        body_value,
        permalink_value,
        author_id,
        --NOW(),
        is_draft,
        FALSE
    ) RETURNING tp_id INTO this_page_id;

    RETURN this_page_id;

END;

$$;
