
pub fn get_os_from_user_agent(
    user_agent_value: &str,
) -> String {

    let os_name =
    if user_agent_value.contains("FreeBSD")      { "FreeBSD" }
    else if user_agent_value.contains("OpenBSD") { "OpenBSD" }
    else if user_agent_value.contains("Android") { "Android" }
    else if user_agent_value.contains("Linux")   { "GNU/Linux" }
    else if user_agent_value.contains("iPhone")
        || user_agent_value.contains("iphone")
        || user_agent_value.contains("iPad")
        || user_agent_value.contains("ipad")     { "iOS" }
    else if user_agent_value.contains("Mac OS")  { "Mac" }
    else if user_agent_value.contains("CrOS")    { "ChromeOS" }
    else if user_agent_value.contains("Windows") { "Windows" }
    else { "unknown" };

    os_name.to_string()

}

