
CREATE FUNCTION trigger_fn_tsm_server()

RETURNS TRIGGER 

LANGUAGE PLPGSQL

AS $$

DECLARE

    date_value TIMESTAMPTZ;

BEGIN

    date_value := DATE_TRUNC('hour', NEW.tsms_date);

    IF c_tshs_date(date_value) THEN

        UPDATE tsh_server
        SET tshs_uploaded = tshs_uploaded + NEW.tsms_uploaded,
            tshs_downloaded = tshs_downloaded + NEW.tsms_downloaded,
            tshs_disk_used = NEW.tsms_disk_used,
            tshs_disk_free = NEW.tsms_disk_free,
            tshs_cpu = s_average_tsms_cpu_by_hour(date_value),
            tshs_memory = s_average_tsms_memory_by_hour(date_value)
        WHERE tshs_date = date_value;

    ELSE

        INSERT INTO tsh_server (
            tshs_date,
            tshs_uploaded,
            tshs_downloaded,
            tshs_disk_used,
            tshs_disk_free,
            tshs_cpu,
            tshs_memory
        ) VALUES (
            date_value,
            NEW.tsms_uploaded,
            NEW.tsms_downloaded,
            NEW.tsms_disk_used,
            NEW.tsms_disk_free,
            NEW.tsms_cpu,
            NEW.tsms_memory
        );

    END IF;

    RETURN NEW;

END;

$$;



CREATE TRIGGER trigger_tsm_server
AFTER INSERT
ON tsm_server
FOR EACH ROW
EXECUTE PROCEDURE trigger_fn_tsm_server();
