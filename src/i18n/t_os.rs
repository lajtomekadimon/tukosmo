
pub fn t_os(
    os_value: &str,
) -> String {

    match os_value {
        "arch" => "Arch Linux",
        "debian" => "Debian",
        "dragonflybsd" => "DragonFlyBSD",
        "fedora" => "Fedora",
        "freebsd" => "FreeBSD",
        "gentoo" => "Gentoo",
        "netbsd" => "NetBSD",
        "openbsd" => "OpenBSD",
        "opensuse" => "OpenSUSE",
        "ubuntu" => "Ubuntu",

        //--------------//

        _ => "(unknown)",
    }.to_string()

}
