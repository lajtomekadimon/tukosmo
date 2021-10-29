
pub fn get_browser_from_user_agent(
    user_agent_value: &str,
) -> String {

    let browser_name =
    if user_agent_value.contains("IceCat")         { "IceCat" }
    else if user_agent_value.contains("Iceweasel") { "Iceweasel" }
    else if user_agent_value.contains("Firefox")   { "Firefox" }
    else if user_agent_value.contains("Brave")     { "Brave" }
    else if user_agent_value.contains("Midori")    { "Midori" }
    else if user_agent_value.contains("Edge")      { "Edge" }
    else if user_agent_value.contains("Opera")     { "Opera" }
    else if user_agent_value.contains("Chrome")    { "Chrome" }
    else if user_agent_value.contains("Safari")    { "Safari" }
    else { "unknown" };

    browser_name.to_string()

}

