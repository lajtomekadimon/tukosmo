use image;
use image::imageops::FilterType;
use std::path::Path;


pub fn resize_image(
    ofile_path: &str,
    nwidth: u32,
    nheight: u32,
    new_file_path: &str,
) {

    let img = image::open(ofile_path).expect("Opening image failed");
    let thumbnail = img.resize(nwidth, nheight, FilterType::Lanczos3);
    thumbnail.save(Path::new(new_file_path)).expect("Saving image failed");

}
