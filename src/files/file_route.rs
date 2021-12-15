
pub fn file_route(
    name: &str,
) -> String {
    "/files/{name}".replace("{name}", name)
}

