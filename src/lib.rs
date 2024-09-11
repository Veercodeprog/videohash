use actix_web::{get, web, HttpResponse, Responder};
use image::open;
use serde::{Deserialize, Serialize};

pub mod dhash;
pub mod phash;
pub mod video;
pub use crate::dhash::compute_dhash;
pub use crate::phash::compute_phash;
pub use crate::video::{extract_frames, extract_frames_using_videotools};

#[derive(Deserialize)]
pub struct VideoUrl {
    pub video_url: String,
}

#[derive(Serialize)]
pub struct HashResponse {
    pub hash: String,
}

// pHash API
#[get("/phash")]
pub async fn phash_api(query: web::Query<VideoUrl>) -> impl Responder {
    match video::extract_frames_using_videotools(&query.video_url) {
        Ok(frame_paths) => {
            let mut hashes = Vec::new();
            for frame_path in frame_paths {
                match open(&frame_path) {
                    Ok(img) => {
                        let phash = compute_phash(&img);
                        hashes.push(phash);
                    }
                    Err(e) => eprintln!("Error opening image file {}: {}", frame_path, e),
                }
            }
            HttpResponse::Ok().json(hashes)
        }
        Err(e) => {
            eprintln!("Error processing video: {}", e);
            HttpResponse::BadRequest().body("Error processing video")
        }
    }
}

// dHash API
#[get("/dhash")]
pub async fn dhash_api(query: web::Query<VideoUrl>) -> impl Responder {
    match extract_frames_using_videotools(&query.video_url) {
        Ok(frame_paths) => {
            let mut hashes = Vec::new();
            for frame_path in frame_paths {
                match open(&frame_path) {
                    Ok(img) => {
                        let dhash = compute_dhash(&img);
                        hashes.push(dhash);
                    }
                    Err(e) => eprintln!("Error opening image file {}: {}", frame_path, e),
                }
            }
            HttpResponse::Ok().json(hashes)
        }
        Err(e) => {
            eprintln!("Error processing video: {}", e);
            HttpResponse::BadRequest().body("Error processing video")
        }
    }
}

// Add more public APIs or utilities as needed.
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use serde_json::Value;
    use std::env;
    #[actix_web::test]
    async fn test_phash_api() {
        // Read the VIDEO_URL environment variable
        let video_url = env::var("VIDEO_URL").unwrap_or_else(|_| "default_video_path".to_string());

        // Create an Actix app with the phash_api route
        let app = test::init_service(App::new().service(phash_api)).await;

        // Send a GET request to the /phash endpoint
        let resp = test::TestRequest::get()
            .uri(&format!("/phash?video_url={}", video_url))
            .send_request(&app)
            .await;

        // Verify the response
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        // Parse the JSON response and print hashes
        let hashes: Vec<String> = serde_json::from_slice(&body).unwrap_or_else(|_| vec![]);
        println!("pHash values: {:?}", hashes);
    }

    #[actix_web::test]
    async fn test_dhash_api() {
        // Read the VIDEO_URL environment variable
        let video_url = env::var("VIDEO_URL").unwrap_or_else(|_| "default_video_path".to_string());

        // Create an Actix app with the dhash_api route
        let app = test::init_service(App::new().service(dhash_api)).await;

        // Send a GET request to the /dhash endpoint
        let resp = test::TestRequest::get()
            .uri(&format!("/dhash?video_url={}", video_url))
            .send_request(&app)
            .await;

        // Verify the response
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        // Parse the JSON response and print hashes
        let hashes: Vec<String> = serde_json::from_slice(&body).unwrap_or_else(|_| vec![]);
        println!("dHash values: {:?}", hashes);
    }
}
