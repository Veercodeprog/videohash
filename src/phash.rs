use image::{DynamicImage, GenericImageView};
use imagehash::PerceptualHash;
use std::time::Instant;
pub fn compute_phash(image: &DynamicImage) -> String {
    let start_time = Instant::now(); // Start timing hashing/fingerprinting

    let hasher = PerceptualHash::new();
    let hash = hasher.hash(image);
    println!("Phash computed: {}", hash.to_string());
    let elapsed_time = start_time.elapsed(); // Stop timing frame extraction
    println!("PHASHing completed in {:?}", elapsed_time);

    return hash.to_string(); // Returns the hex-encoded hash string
}
