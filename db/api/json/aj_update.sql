
CREATE OR REPLACE FUNCTION aj_update(

    input_json JSONB,

    session_id UUID

)

RETURNS JSONB

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    jval_transaction JSONB;

    time_before     TIMESTAMPTZ;
    time_after      TIMESTAMPTZ;
    exec_time_value DOUBLE PRECISION;

    output_json JSONB;

    m TEXT;

    error_code TEXT;
    error_id UUID;
    error_loc TEXT;

    message_value_i18n TEXT;

BEGIN

    time_before := CLOCK_TIMESTAMP();

    -- Check input errors
    --PERFORM some_random_function(input_json);

    -- Update database
    --PERFORM some_random_function(input_json);

    time_after := CLOCK_TIMESTAMP();

    exec_time_value := EXTRACT(EPOCH FROM (time_after - time_before));

    -- This should be improved for sure
    output_json := JSONB_BUILD_OBJECT(
        'success', TRUE,
        'exec_time', exec_time_value
    );

    RETURN output_json;

EXCEPTION WHEN SQLSTATE 'P0001' THEN

    GET STACKED DIAGNOSTICS m = MESSAGE_TEXT;

    error_code := m; --SUBSTRING(m FROM 1 FOR 5);

    time_after := CLOCK_TIMESTAMP();

    exec_time_value := EXTRACT(EPOCH FROM (time_after - time_before));

    output_json := JSONB_BUILD_OBJECT(
        'success', FALSE,
        'error', error_code,
        'exec_time', exec_time_value
    );

    RETURN output_json;

WHEN OTHERS THEN

    GET STACKED DIAGNOSTICS m = MESSAGE_TEXT;

    error_code := 'UNKNOWN ERROR CODE';

    time_after := CLOCK_TIMESTAMP();

    exec_time_value := EXTRACT(EPOCH FROM (time_after - time_before));

    output_json := JSONB_BUILD_OBJECT(
        'success', FALSE,
        'error', error_code,
        'exec_time', exec_time_value
    );

    RETURN output_json;

END;

$$;
