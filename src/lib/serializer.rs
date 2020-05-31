use super::worker::ImageDimensions;
use super::worker::ReadError;

fn error_json(file_path: String, error: &str) -> String {
    format!(r#"{{"error":"{}","file":"{}"}}"#, error, file_path)
}

fn dimensions_json(image_dimensions: ImageDimensions) -> String
{
    format!(
        r#"{{"file":"{}","width":{},"height":{},"type":"{}"}}"#,
        image_dimensions.src,
        image_dimensions.width,
        image_dimensions.height,
        image_dimensions.mime_type()
    )
}

pub fn serialize_event(event: Result<ImageDimensions, ReadError>) -> String {
    match event {
        Err(ReadError::FileNotFound(file)) => error_json(file, "File was not found"),
        Err(ReadError::NotImage(file)) => error_json(file, "File is not an image"),
        Err(ReadError::CorruptedImage(file)) => error_json(file, "Image file is corrupted"),
        Ok(dimensions) => dimensions_json(
            dimensions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::worker::ImageType;

    #[test]
    fn it_serializes_file_not_found_read_error()
    {
        assert_eq!(
            r#"{"error":"File was not found","file":"file_not_found.jpg"}"#,
            serialize_event(Err(ReadError::FileNotFound("file_not_found.jpg".to_owned())))
        );
    }

    #[test]
    fn it_serializes_not_an_image_read_error()
    {
        assert_eq!(
            r#"{"error":"File is not an image","file":"file.text"}"#,
            serialize_event(Err(ReadError::NotImage("file.text".to_owned())))
        )
    }

    #[test]
    fn it_serializes_corrupted_image_read_error()
    {
        assert_eq!(
            r#"{"error":"Image file is corrupted","file":"corrupted.png"}"#,
            serialize_event(Err(ReadError::CorruptedImage("corrupted.png".to_owned())))
        )
    }

    #[test]
    fn it_serializes_jpeg_image_dimensions()
    {
        assert_eq!(
            r#"{"file":"some-file.jpg","width":680,"height":299,"type":"image/jpeg"}"#,
            serialize_event(
                Ok(
                    ImageDimensions::new(
                        String::from("some-file.jpg"),
                        680,
                        299,
                        ImageType::JPEG
                    )
                )
            )
        )
    }

    #[test]
    fn it_serializes_png_image_dimensions()
    {
        assert_eq!(
            r#"{"file":"some-file.png","width":200,"height":100,"type":"image/png"}"#,
            serialize_event(
                Ok(
                    ImageDimensions::new(
                        String::from("some-file.png"),
                        200,
                        100,
                        ImageType::PNG
                    )
                )
            )
        )
    }

    #[test]
    fn it_serializes_gif_image_dimensions()
    {
        assert_eq!(
            r#"{"file":"some-file.gif","width":100,"height":200,"type":"image/gif"}"#,
            serialize_event(
                Ok(
                    ImageDimensions::new(
                        String::from("some-file.gif"),
                        100,
                        200,
                        ImageType::GIF
                    )
                )
            )
        )
    }

    #[test]
    fn it_serializes_webp_image_dimensions()
    {
        assert_eq!(
            r#"{"file":"some-file.webp","width":1280,"height":720,"type":"image/webp"}"#,
            serialize_event(
                Ok(
                    ImageDimensions::new(
                        String::from("some-file.webp"),
                        1280,
                        720,
                        ImageType::WEBP
                    )
                )
            )
        )
    }

    #[test]
    fn it_serializes_generic_image_dimensions()
    {
        assert_eq!(
            r#"{"file":"some-file.bmp","width":1280,"height":720,"type":"image/generic"}"#,
            serialize_event(
                Ok(
                    ImageDimensions::new(
                        String::from("some-file.bmp"),
                        1280,
                        720,
                        ImageType::GENERIC
                    )
                )
            )
        )
    }
}