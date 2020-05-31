use tokio::fs;
use tokio::prelude::*;
use imagesize::{image_type, blob_size, ImageType as ImageSizeType};
use crate::lib::worker::ReadError::{FileNotFound, NotImage, CorruptedImage};

#[derive(PartialEq, Debug)]
pub struct InputCommand(String);

#[derive(PartialEq, Debug)]
pub struct ImageDimensions {
    pub src: String,
    pub width: usize,
    pub height: usize,
    pub image_type: ImageType
}

impl ImageDimensions {
    pub fn new(src: String, width: usize, height: usize, image_type: ImageType) -> Self
    {
        ImageDimensions {
            src,
            width,
            height,
            image_type
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ImageType {
    JPEG,
    PNG,
    GIF,
    WEBP,
    GENERIC
}

#[derive(PartialEq, Debug)]
pub enum ReadError {
    FileNotFound(String),
    NotImage(String),
    CorruptedImage(String)
}

pub async fn process_command(command: InputCommand) -> Result<ImageDimensions, ReadError> {
    let file_name = command.0;

    match fs::metadata(&file_name).await {
        Ok(metadata) if (metadata.is_file()) => retrieve_image_dimensions(
            file_name
        ).await,
        _ => Err(FileNotFound(file_name))
    }
}

async fn retrieve_image_dimensions(file_name: String) -> Result<ImageDimensions, ReadError> {
    let mut file: fs::File = fs::OpenOptions::new().read(true).open(&file_name).await.unwrap();
    let mut file_data = [0; 5_120];

    let bytes_read = file.read(&mut file_data[..]).await.unwrap();

    let image_info = (
        image_type(&file_data[..bytes_read]),
        blob_size(&file_data[..bytes_read])
    );

    match image_info {
        (Ok(image_type), Ok(image_size)) => Ok(ImageDimensions::new(
            file_name,
            image_size.width,
            image_size.height,
            map_image_type(image_type)
        )),
        (Err(_), _) => Err(NotImage(file_name)),
        (Ok(_), Err(_)) => Err(CorruptedImage(file_name))
    }
}

fn map_image_type(image_size_type: ImageSizeType) -> ImageType {
    match image_size_type {
        ImageSizeType::Webp => ImageType::WEBP,
        ImageSizeType::Png => ImageType::PNG,
        ImageSizeType::Jpeg => ImageType::JPEG,
        ImageSizeType::Gif => ImageType::GIF,
        _ => ImageType::GENERIC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_path(relative: &str) -> String {
        return std::env::current_dir().unwrap()
            .join("fixtures")
            .join(relative)
            .to_str()
            .unwrap()
            .to_owned();
    }

    #[tokio::test]
    async fn it_returns_file_not_found_event() {
        assert_eq!(
            ReadError::FileNotFound(fixture_path("file_that_does_not_exists.jpg")),
            process_command(InputCommand(fixture_path("file_that_does_not_exists.jpg")))
                .await.unwrap_err()
        )
    }

    #[tokio::test]
    async fn it_returns_jpeg_image_dimensions() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("720p.jpg"),
                1280,
                720,
                ImageType::JPEG
            ),

            process_command(InputCommand(fixture_path("720p.jpg")))
                .await.unwrap()
        )
    }

    #[tokio::test]
    async fn it_returns_png_image_dimensions() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("720p.png"),
                1280,
                720,
                ImageType::PNG
            ),

            process_command(InputCommand(fixture_path("720p.png")))
                .await.unwrap()
        )
    }

    #[tokio::test]
    async fn it_returns_webp_image_dimensions() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("720p.webp"),
                1280,
                720,
                ImageType::WEBP
            ),

            process_command(InputCommand(fixture_path("720p.webp")))
                .await.unwrap()
        )
    }

    #[tokio::test]
    async fn it_returns_gif_image_dimensions() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("720p.gif"),
                1280,
                720,
                ImageType::GIF
            ),

            process_command(InputCommand(fixture_path("720p.gif")))
                .await.unwrap()
        )
    }


    #[tokio::test]
    async fn it_returns_smaller_image_dimensions() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("360p.webp"),
                640,
                360,
                ImageType::WEBP
            ),

            process_command(InputCommand(fixture_path("360p.webp")))
                .await.unwrap()
        )
    }

    #[tokio::test]
    async fn it_returns_generic_image_type_if_type_is_not_supported() {
        assert_eq!(
            ImageDimensions::new(
                fixture_path("720p.bmp"),
                1280,
                720,
                ImageType::GENERIC
            ),

            process_command(InputCommand(fixture_path("720p.bmp")))
                .await.unwrap()
        )
    }

    #[tokio::test]
    async fn it_returns_file_is_not_an_image_error_when_wrong_file_type_is_provided() {
        assert_eq!(
            ReadError::NotImage(fixture_path("file.txt")),

            process_command(InputCommand(fixture_path("file.txt")))
                .await.unwrap_err()
        )
    }

    #[tokio::test]
    async fn it_returns_image_is_corrupted_error_when_size_cannot_be_read() {
        assert_eq!(
            ReadError::CorruptedImage(fixture_path("corrupted.png")),

            process_command(InputCommand(fixture_path("corrupted.png")))
                .await.unwrap_err()
        )
    }
}