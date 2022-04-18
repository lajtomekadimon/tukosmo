use chrono::{DateTime, Utc};

use crate::i18n::t_date_chrono::t_date_chrono;


pub fn t_date(
    date_value: &str,
    lang_code: &str,
) -> String {

    match date_value {
        "" => "".to_string(),
        _ => {
            // Convert the string into DateTime<FixedOffset>
            let datetime = DateTime::parse_from_str(
                date_value,
                "%Y-%m-%d %H:%M:%S%.f%#z"
            ).unwrap();

            // Convert the string into DateTime<Utc> or other timezone
            let datetime_utc = datetime.with_timezone(&Utc);

            t_date_chrono(
                &datetime_utc,
                lang_code,
            )
        }
    }

}
