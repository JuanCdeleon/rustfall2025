use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fmt;

#[derive(Debug)]
enum DogApiError {
    NetworkError(String),
    IoError(String),
}

impl fmt::Display for DogApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DogApiError::NetworkError(e) => write!(f, "Network Error: {}", e),
            DogApiError::IoError(e) => write!(f, "I/O Error: {}", e),
        }
    }
}

impl Error for DogApiError {}

fn download_image(url: &str, file_name: &str) -> Result<(), DogApiError> {
    println!("Downloading image from: {}", url);
    let mut response = ureq::get(url)
        .call()
        .map_err(|e| DogApiError::NetworkError(format!("Request failed: {}", e)))?;

    let mut bytes: Vec<u8> = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| DogApiError::IoError(format!("Failed to read response: {}", e)))?;

    let mut file = File::create(file_name)
        .map_err(|e| DogApiError::IoError(format!("Failed to create file: {}", e)))?;
    file.write_all(&bytes)
        .map_err(|e| DogApiError::IoError(format!("Failed to write to file: {}", e)))?;

    println!("✅ Image saved as '{}'\n", file_name);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("=======================\n");

    
    let urls = vec![
        "https://images.dog.ceo/breeds/dingo/n02115641_5178.jpg",
        "https://images.dog.ceo/breeds/terrier-irish/n02093991_1350.jpg",
        "https://images.dog.ceo/breeds/poodle-miniature/n02113712_3384.jpg",
        "https://images.dog.ceo/breeds/husky/n02110185_1469.jpg",
        "https://images.dog.ceo/breeds/labrador/n02099712_1506.jpg",
    ];

    for (i, url) in urls.iter().enumerate() {
        let file_name = format!("dog{}.jpg", i + 1);
        match download_image(url, &file_name) {
            Ok(_) => println!("✅ Successfully downloaded {}\n", file_name),
            Err(e) => println!("❌ Failed to download {}: {}\n", file_name, e),
        }
    }

    println!("All downloads complete!");
    Ok(())
}
