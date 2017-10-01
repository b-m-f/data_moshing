extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new("test.jpg")).expect("Could not load Image");

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    let ref mut fout = File::create(&Path::new("test.png")).expect("Could not create Image");

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(fout, image::PNG).expect("Could not write the file to Disk");
}

