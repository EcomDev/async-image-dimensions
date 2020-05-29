use super::worker::ImageDimensions;
use super::worker::ReadError;

fn error_json(file_path: String, error: &str) -> String {
    format!(r#"{{"error":"{}","file":"{}"}}"#, error, file_path)
}

pub fn serialize_event(event: Result<ImageDimensions, ReadError>) -> String {
    match event {
        Err(ReadError::FileNotFound(file)) => error_json(file, "File was not found"),
        Err(ReadError::NotImage(file)) => error_json(file, "File is not an image"),
        Err(ReadError::CorruptedImage(file)) => error_json(file, "Image file is corrupted"),
        _ => "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}