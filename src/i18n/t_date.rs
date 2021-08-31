use chrono::{DateTime, Utc, Datelike, Timelike};

use crate::i18n::month_name::month_name;


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

            let year = datetime_utc.year();
            let month = datetime_utc.month();
            let day = datetime_utc.day();
            let _hour = datetime_utc.hour();
            let _minute = datetime_utc.minute();

            match lang_code {
                "en" => "{month} {day}, {year}",
                "es" => "{day} de {month} de {year}",

                //--------------//

                //_ => "[unstranslated]"
                //_ => t_date(date_value, "en"),  // doesn't work
                _ => "{month} {day}, {year}",
            }
            .replace("{month}", month_name(month, lang_code))
            .replace("{day}", &day.to_string())
            .replace("{year}", &year.to_string())
            .to_string()
        }
    }

}
