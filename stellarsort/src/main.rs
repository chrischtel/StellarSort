use stellarsort_core::detect_blur;

fn main() {
    let img = image::open("blur_noise.png").unwrap();

    let blur_threshold = 100.0;  // Adjust this based on testing

    // Check if the image is blurry
    if detect_blur(img, blur_threshold, 1.0) {
        println!("The image is blurry.");
    } else {
        println!("The image is sharp.");
    }
}
