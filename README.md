# video hash crate

# VideoHash Crate

The `videohash` crate provides functionality for computing perceptual hashes (pHash) and difference hashes (dHash) from video files. This crate extracts frames from videos and computes these hashes for each frame.

## Functions

### `compute_phash`

**Description**: Computes a perceptual hash (pHash) for a video by extracting frames and computing the hash for each extracted frame.

**Parameters**:

- `video_path` (type: `&str`): Path to the video file.

**Returns**: `Result<Vec<String>, Box<dyn Error>>` - A list of perceptual hashes for five frames extracted from the video.

**Usage Example**:

```rust
rust
Copy code
use videohash::compute_phash;

fn main() -> Result<(), Box<dyn Error>> {
    let video_path = "path/to/video.mp4";
    let phashes = compute_phash(video_path)?;
    for phash in phashes {
        println!("pHash: {}", phash);
    }
    Ok(())
}

```

**Flow**:

1. **Extract Frames**: The function extracts five frames from the video at evenly spaced intervals.
2. **Compute pHash**: Computes the perceptual hash for each of the five frames.
3. **Return**: Returns a list of the computed pHashes.

### `compute_dhash`

**Description**: Computes a difference hash (dHash) for a video by extracting frames and computing the hash for each extracted frame.

**Parameters**:

- `video_path` (type: `&str`): Path to the video file.

**Returns**: `Result<Vec<String>, Box<dyn Error>>` - A list of difference hashes for five frames extracted from the video.

**Usage Example**:

```rust

use videohash::compute_dhash;

fn main() -> Result<(), Box<dyn Error>> {
    let video_path = "path/to/video.mp4";
    let dhashes = compute_dhash(video_path)?;
    for dhash in dhashes {
        println!("dHash: {}", dhash);
    }
    Ok(())
}

```

**Flow**:

1. **Extract Frames**: The function extracts five frames from the video at evenly spaced intervals.
2. **Compute dHash**: Computes the difference hash for each of the five frames.
3. **Return**: Returns a list of the computed dHashes.

## API Endpoints

### `/phash`

**Description**: Computes perceptual hashes for five frames extracted from a video.

**Parameters**:

- `video_url` (type: `String`): URL or path to the video file.

**Returns**: JSON array of perceptual hashes for the extracted frames.

**Example Request**:

```

GET /phash?video_url=path/to/video.mp4

```

**Example Response**:

```json
["hash1", "hash2", "hash3", "hash4", "hash5"]
```

### `/dhash`

**Description**: Computes difference hashes for five frames extracted from a video.

**Parameters**:

- `video_url` (type: `String`): URL or path to the video file.

**Returns**: JSON array of difference hashes for the extracted frames.

**Example Request**:

```

GET /dhash?video_url=path/to/video.mp4

```

**Example Response**:

```json
["hash1", "hash2", "hash3", "hash4", "hash5"]
```

## Testing

To run tests for the API endpoints, ensure you have a video URL set in your environment variables:

```

export VIDEO_URL="path/to/video.mp4"
cargo test

```

## License

This crate is licensed under the MIT License. See the LICENSE file for more details.

## Contributions

Contributions are welcome! Please open an issue or submit a pull request on the GitHub repository.

```css

This README provides a clear explanation of the `compute_phash` and `compute_dhash` functions, their usage, and their role in the crate. It also includes information about the API endpoints and how to test the functionality.

```
