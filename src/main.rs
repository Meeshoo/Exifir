use std::fmt::Debug;
use std::path::PathBuf;

use iced::keyboard::key::Named::New;
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use iced::widget::{button, column, container, image, row, text, Column};
use iced::{Fill, Shrink};
use little_exif::u8conversion::U8conversion;
use rfd::FileDialog;

struct Viewer {
    image_path: String,
    image_filename: String,
    metadata_datetime_created: String,
    metadata_camera_brand: String,
    metadata_camera_model: String,
    metadata_user_comment: String,
    metadata_gps_latitude: String,  //-0.137348
    metadata_gps_longitude: String, //50.819250
}

struct ImageMetadata {
    datetime_created: String,
    camera_brand: String,
    camera_model: String,
    user_comment: String,
    gps_latitude: String,
    gps_longitude: String,
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

impl ImageMetadata {
    fn new() -> Self {
        ImageMetadata {
            datetime_created: String::new(),
            camera_brand: String::new(),
            camera_model: String::new(),
            user_comment: String::new(),
            gps_latitude: String::new(),
            gps_longitude: String::new(),
        }
    }
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
                self.image_path = image_path
                    .to_str()
                    .expect("Failed to get image path")
                    .into();
                // Todo: Fix this abomination
                self.image_filename = image_path
                    .file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .expect("Failed to cast file name to string")
                    .to_string();

                let metadata = get_metadata(image_path);

                self.metadata_datetime_created = metadata.datetime_created;
                self.metadata_camera_brand = metadata.camera_brand;
                self.metadata_camera_model = metadata.camera_model;
                self.metadata_user_comment = metadata.user_comment;
                self.metadata_gps_latitude = metadata.gps_latitude;
                self.metadata_gps_longitude = metadata.gps_longitude;
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
            .height(20)
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
        .set_directory("Pictures/")
        .set_title("Choose an image...")
        .pick_file()
        .expect("Failed to get file");
    file_handle.as_path().into()
}

// fn load_folder() -> PathBuf {
//     let file_handle = FileDialog::new()
//         .add_filter("image", &["png", "jpg", "jpeg"])
//         .set_directory("/home")
//         .set_title("Choose a folder...")
//         .pick_folder()
//         .expect("Failed to get folder");
//     file_handle
// }

fn get_metadata(image_path: PathBuf) -> ImageMetadata {
    let mut image_metadata: ImageMetadata = ImageMetadata::new();

    let metadata: Metadata =
        Metadata::new_from_path(image_path.as_path()).expect("Failed to get metadata");

    let endian = metadata.get_endian();

    let datetime_created = metadata
        .get_tag(&&ExifTag::DateTimeOriginal(String::new()))
        .next()
        .expect("Can't get datetime");

    image_metadata.datetime_created = String::from_u8_vec(
        &datetime_created.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let camera_brand = metadata
        .get_tag(&&ExifTag::Make(String::new()))
        .next()
        .expect("Can't get datetime");

    image_metadata.camera_brand = String::from_u8_vec(
        &camera_brand.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let camera_model = metadata
        .get_tag(&&ExifTag::Model(String::new()))
        .next()
        .expect("Can't get user comment");

    image_metadata.camera_model = String::from_u8_vec(
        &camera_model.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let user_comment = metadata
        .get_tag(&&ExifTag::UserComment(Vec::new()))
        .next()
        .expect("Can't get user comment");

    image_metadata.user_comment = String::from_u8_vec(
        &user_comment.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let gps_longitude = metadata
        .get_tag(&&&ExifTag::GPSLongitude(Vec::new()))
        .next()
        .expect("Can't get datetime");

    image_metadata.gps_longitude = String::from_u8_vec(
        &gps_longitude.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let gps_latitude = metadata
        .get_tag(&&&ExifTag::GPSLatitude(Vec::new()))
        .next()
        .expect("Can't get user comment");

    image_metadata.gps_latitude = String::from_u8_vec(
        &gps_latitude.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    image_metadata
}

pub fn main() -> iced::Result {
    iced::application(Viewer::new, Viewer::update, Viewer::view)
        .title("Exifir")
        .run()
}
