use image::{DynamicImage, GenericImageView};
use imagehash::DifferenceHash;
use std::time::Instant;
pub fn compute_dhash(image: &DynamicImage) -> String {
    // Create a DifferenceHash instance with default parameters
    let start_time = Instant::now(); // Start timing hashing/fingerprinting

    let hasher = DifferenceHash::new();

    // Compute the dHash for the image
    let hash = hasher.hash(image);

    // Print and return the hex-encoded hash string
    println!("{}", hash.to_string());
    let elapsed_time = start_time.elapsed(); // Stop timing frame extraction
    println!("DHASHing completed in {:?}", elapsed_time);
    hash.to_string()
}
