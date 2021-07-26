
CREATE OR REPLACE FUNCTION aj_query(

    input_json JSONB

)

RETURNS JSONB

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    error_msg TEXT;

    time_before     TIMESTAMPTZ;
    time_after      TIMESTAMPTZ;
    exec_time_value DOUBLE PRECISION;

    output_json JSONB;

BEGIN

    time_before := CLOCK_TIMESTAMP();

    --[OLD]error_msg := val_input_api_download(input_json);

    --[OLD]IF error_msg IS NULL THEN

        output_json := some_other_function(input_json);

        output_json := JSONB_INSERT(
            output_json,
            ARRAY['success'],
            TO_JSONB(TRUE)
        );

        time_after := CLOCK_TIMESTAMP();

        exec_time_value := EXTRACT(EPOCH FROM (time_after - time_before));

        output_json := JSONB_INSERT(
            output_json,
            ARRAY['exec_time'],
            TO_JSONB(exec_time_value)
        );

    /*[OLD]ELSE

        time_after := CLOCK_TIMESTAMP();

        exec_time_value := EXTRACT(EPOCH FROM (time_after - time_before));

        output_json := JSONB_BUILD_OBJECT(
            'success', FALSE,
            'error', error_msg,
            'exec_time', exec_time_value
        );

    END IF;*/

    RETURN output_json;

END;

$$;
