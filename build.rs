use image::open;
use std::{fs, path::Path};

fn main() {
    let image_dir = "public/images";
    let target_dir = "target/site/images";
    // if the contents changed, rerun the build
    println!("cargo:rerun-if-changed={image_dir}");
    optimize_images(image_dir, target_dir);
}

fn optimize_images(image_dir: &str, target_dir: &str) {
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).expect("Failed to create target directory");
    }
    fs::read_dir(image_dir)
        .expect("Failed to read image directory")
        .for_each(|file| {
            let input_file = file.expect("Cannot read image");
            let original_path =
                format!("{}/{}", image_dir, input_file.file_name().to_str().unwrap());
            let file_full_name = input_file.file_name();
            let file_elements = file_full_name
                .to_str()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>();
            if let Some(file_name) = file_elements.first() {
                let path = format!("{}/{}.webp", target_dir, file_name);
                let img = open(original_path).expect("Open file buffer failed");
                img.save(path).expect("convert image to webp failed");
            }
        });
}
