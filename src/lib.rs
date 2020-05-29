use tokio::fs;
use tokio::prelude::*;
use imagesize;
use std::cmp::min;

#[derive(Eq, PartialEq, Debug)]
pub struct InputCommand(String);

#[derive(Eq, PartialEq, Debug)]
pub enum OutputEvent {
    FileNotFound(String),
    ImageDimensions {
        src: String,
        width: usize,
        height: usize,
        mime_type: String
    }
}

impl InputCommand {
    fn from(input: &str) -> InputCommand {
        InputCommand(input.to_owned())
    }
}

impl OutputEvent
{
    fn file_not_found(filename: String) -> OutputEvent {
        OutputEvent::FileNotFound(filename)
    }

    fn image_dimensions(filename: String, width: usize, height: usize, mime_type: String) -> OutputEvent {
        OutputEvent::ImageDimensions {
            src: filename,
            width,
            height,
            mime_type
        }
    }
}

pub async fn process_command(command: InputCommand) -> OutputEvent {
    let file_name = command.0;

    match fs::metadata(&file_name).await {
        Ok(metadata) if (metadata.is_file()) => retrieve_image_dimensions(
            file_name
        ).await,
        _ => OutputEvent::FileNotFound(file_name)
    }
}

async fn retrieve_image_dimensions(fileName: String) -> OutputEvent {
    let mut file: fs::File = fs::OpenOptions::new().read(true).open(&fileName).await.unwrap();
    let mut fileData = [0; 5_120];

    let readSize = file.read(&mut fileData[..]).await.unwrap();
    let size = imagesize::blob_size(&fileData[..readSize]).unwrap();

    OutputEvent::image_dimensions(
        fileName,
        size.width,
        size.height,
        "image/jpeg".to_owned()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;
    use std::env;

    fn fixture_path(relative: &str) -> String {
        let parentPath = std::env::current_dir().unwrap();

        return parentPath
            .join("fixtures")
            .join(relative)
            .to_str()
            .unwrap()
            .to_owned();
    }

    mod process_input_command {
        use super::*;

        #[tokio::test]
        async fn it_returns_file_not_found_event() {
            assert_eq!(
                OutputEvent::file_not_found(fixture_path("file_that_does_not_exists.jpg")),
                process_command(InputCommand(fixture_path("file_that_does_not_exists.jpg")))
                    .await
            )
        }

        #[tokio::test]
        async fn it_returns_jpeg_image_dimensions() {
            assert_eq!(
                OutputEvent::image_dimensions(
                    fixture_path("720p.jpg"),
                    1280,
                    720,
                    "image/jpeg".to_owned()
                ),

                process_command(InputCommand(fixture_path("720p.jpg")))
                    .await
            )
        }
    }

}