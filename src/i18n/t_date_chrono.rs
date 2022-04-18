use chrono::{DateTime, Utc, Datelike, Timelike};

use crate::i18n::month_name::month_name;


pub fn t_date_chrono(
    datetime_utc: &DateTime<Utc>,
    lang_code: &str,
) -> String {

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
