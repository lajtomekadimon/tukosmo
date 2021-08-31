
pub fn month_name(
    month_number: u32,
    lang_code: &str,
) -> &str {

    match lang_code {
        "en" => match month_number {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "(none)",
        }

        "es" => match month_number {
            1 => "enero",
            2 => "febrero",
            3 => "marzo",
            4 => "abril",
            5 => "mayo",
            6 => "junio",
            7 => "julio",
            8 => "agosto",
            9 => "septiembre",
            10 => "octubre",
            11 => "noviembre",
            12 => "diciembre",
            _ => "(none)",
        }

        //--------------//

        //_ => "[unstranslated]"
        _ => month_name(month_number, "en"),
    }

}
