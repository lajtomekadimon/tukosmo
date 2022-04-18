
// TODO: Use . or , depending on language
pub fn t_bytesize(
    nbytes: &i64,
) -> (f64, String) {

    if nbytes < &1024 {
        (*nbytes as f64, "bytes".to_string())
    } else if nbytes < &1048576 {
        ((*nbytes as f64) / 1024.0, "KiB".to_string())
    } else if nbytes < &1073741824 {
        ((*nbytes as f64) / 1048576.0, "MiB".to_string())
    } else {
        ((*nbytes as f64) / 1073741824.0, "GiB".to_string())
    }

}
