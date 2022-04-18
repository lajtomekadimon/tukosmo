use crate::i18n::translate_i18n::TranslateI18N;


pub fn t_duration(
    secs: &i64,
    t: &TranslateI18N,
) -> String {

    if secs == &1 {
        t.one_second.to_string()
    } else if secs < &60 {
        t.n_seconds_w_time.replace(
            "{time}",
            &secs.to_string(),
        )
    } else if secs < &120 {
        t.one_minute.to_string()
    } else if secs < &3600 {
        t.n_minutes_w_time.replace(
            "{time}",
            &(secs / 60).to_string(),
        )
    } else if secs < &7200 {
        t.one_hour.to_string()
    } else if secs < &86400 {
        t.n_hours_w_time.replace(
            "{time}",
            &(secs / 3600).to_string(),
        )
    } else if secs < &172800 {
        t.one_day.to_string()
    } else if secs < &2592000 {
        t.n_days_w_time.replace(
            "{time}",
            &(secs / 86400).to_string(),
        )
    } else if secs < &5184000 {
        t.one_month.to_string()
    } else if secs < &31536000 {
        t.n_months_w_time.replace(
            "{time}",
            &(secs / 2592000).to_string(),
        )
    } else if secs < &63072000 {
        t.one_year.to_string()
    } else {
        t.n_years_w_time.replace(
            "{time}",
            &(secs / 31536000).to_string(),
        )
    }

}
