
pub fn file_size_in_units(
    nbytes: i64,
) -> String {
    let mut n = nbytes.clone() as f64;
    let mut unit = "bytes";

    if nbytes >= 1024 {
        if nbytes >= 1048576 {
            if nbytes >= 1073741824 {
                n = (nbytes as f64) / 1073741824.0;
                unit = "GiB";
            } else {
                n = (nbytes as f64) / 1048576.0;
                unit = "MiB";
            }
        } else {
            n = (nbytes as f64) / 1024.0;
            unit = "KiB";
        }
    }

    let filesize_in_units = format!("{:.1} {}", n, unit);

    filesize_in_units
}
