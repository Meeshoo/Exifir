use std::path::PathBuf;

use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use iced::widget::{button, column, container, image, row, text, Column};
use iced::{Fill, Shrink};
use rfd::FileDialog;

struct Viewer {
    image_path: String,
    image_filename: String,
    metadata_datetime_created: String,
    metadata_camera_brand: String,
    metadata_camera_model: String,
    metadata_user_comment : String,
    metadata_gps_latitude: String, //-0.137348
    metadata_gps_longitude: String, //50.819250 
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFile,
    Reset,
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
}

impl Viewer {
    fn new() -> Self {
        Viewer {
            image_path: String::from(""),
            image_filename: String::from(""),
            metadata_datetime_created: String::from(""),
            metadata_camera_brand: String::from(""),
            metadata_camera_model: String::from(""),
            metadata_user_comment: String::from(""),
            metadata_gps_latitude: String::from(""),
            metadata_gps_longitude: String::from(""),
        }
    }

    // fn theme(&self) -> Theme {
    //     Theme::Light
    // }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                let image_path: PathBuf = load_image();
                self.image_path = image_path.to_str().expect("oops").into();
                // Todo: Fix this abomination
                self.image_filename = image_path
                    .file_name()
                    .expect("oops")
                    .to_str()
                    .expect("oops")
                    .to_string();

                // let mut metadata: Metadata = Metadata::new_from_path(&self.image_path).expect("ops");
                // self.metadata_datetime_created = metadata.get_tag(ExifTag::CreateDate(()))
                self.metadata_datetime_created = String::from("2023-11-23 11:11:11");
                self.metadata_camera_brand = String::from("Hasselblad");
                self.metadata_camera_model = String::from("500C");
                self.metadata_user_comment = String::from("Fomapan 400");
                self.metadata_gps_latitude = String::from("-0.137348");
                self.metadata_gps_longitude = String::from("50.819250");
            }
            Message::Reset => {
                self.image_path = String::from("");
                self.image_filename = String::from("");
                self.metadata_datetime_created = String::from("");
                self.metadata_camera_brand = String::from("");
                self.metadata_camera_model = String::from("");
                self.metadata_user_comment = String::from("");
                self.metadata_gps_latitude = String::from("");
                self.metadata_gps_longitude = String::from("");
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        // MENU
        let open_file_button = button("Open File").on_press(Message::OpenFile);
        let reset_button = button("Reset").on_press(Message::Reset);
        let menu = row![open_file_button, reset_button].spacing(10);

        // IMAGE PANEL
        let image_name: text::Text = text(&self.image_filename)
            .height(Shrink)
            .width(Fill)
            .center();

        let image: image::Image = image(&self.image_path).width(Fill);

        let image_container = column!(image, image_name).spacing(10);

        let image_panel = container(image_container)
            .style(container::bordered_box)
            .height(Fill)
            .width(Fill)
            .center(Fill);

        // METADATA PANEL
        let metadata_datetime_created: text::Text =
            text("Date Created: ".to_owned() + &self.metadata_datetime_created)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_camera_brand: text::Text =
            text("Camera Brand: ".to_owned() + &self.metadata_camera_brand)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_camera_model: text::Text =
            text("Camera Model: ".to_owned() + &self.metadata_camera_model)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_user_comment: text::Text =
            text("User Comment: ".to_owned() + &self.metadata_user_comment)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_gps_latitude: text::Text =
            text("GPS Latitude: ".to_owned() + &self.metadata_gps_latitude)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_gps_longitude: text::Text =
            text("GPS Longitude: ".to_owned() + &self.metadata_gps_longitude)
                .height(Shrink)
                .width(Shrink)
                .center();

        let metadata_container: Column<'_, Message> = column!(
            metadata_datetime_created,
            metadata_camera_brand,
            metadata_camera_model,
            metadata_user_comment,
            metadata_gps_latitude,
            metadata_gps_longitude
        )
        .width(Shrink)
        .spacing(10)
        .padding(50);

        let metadata_panel: container::Container<'_, _> = container(metadata_container)
            .style(container::bordered_box)
            .height(Fill)
            .width(300)
            .center(Shrink);

        let main_workspace = row!(image_panel, metadata_panel);

        let interface = column!(menu, main_workspace).padding(10).spacing(10);
        interface
    }
}

fn load_image() -> PathBuf {
    let file_handle = FileDialog::new()
        .add_filter("image", &["png", "jpg", "jpeg"])
        .set_directory("/home")
        .set_title("Choose an image...")
        .pick_file()
        .expect("Oops");
    file_handle.as_path().into()
}

// fn load_folder() -> PathBuf {
//     let file_handle = FileDialog::new()
//         .add_filter("image", &["png", "jpg", "jpeg"])
//         .set_directory("/home")
//         .set_title("Choose a folder...")
//         .pick_folder()
//         .expect("Oops");
//     file_handle
// }

pub fn main() -> iced::Result {
    iced::application(Viewer::new, Viewer::update, Viewer::view)
        .title("Exifir")
        .run()
}
