use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use iced::widget::{button, column, container, grid, image, row, scrollable, text, Column};
use iced::{Fill, Shrink};
use little_exif::u8conversion::U8conversion;
use rfd::FileDialog;

struct Viewer {
    path_of_all_images: Vec<String>,
    metadata_datetime_created: String,
    metadata_camera_brand: String,
    metadata_camera_model: String,
    metadata_user_comment: String,
    metadata_gps_latitude: String,
    metadata_gps_longitude: String,
}

struct ImageMetadata {
    datetime_created: String,
    camera_brand: String,
    camera_model: String,
    user_comment: String,
    gps_latitude: String,
    gps_longitude: String,
}

struct ImageContainer {
    image_path: String,
    image_filename: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFolder,
    OpenFiles,
    CloseFolder,
}

// #[derive(Debug, Clone)]
// enum Error {
//     DialogClosed,
//     MetadataGetFailed,
// }

impl ImageContainer {
    fn new() -> Self {
        ImageContainer {
            image_path: String::from(""),
            image_filename: String::from(""),
        }
    }
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
            path_of_all_images: Vec::new(),
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
            Message::OpenFolder => {
                self.path_of_all_images = load_folder();

                let metadata = get_metadata(PathBuf::from(
                    "/home/mitch/Pictures/EXIFIR_TEST/000086280005.jpg",
                ));

                self.metadata_datetime_created = metadata.datetime_created;
                self.metadata_camera_brand = metadata.camera_brand;
                self.metadata_camera_model = metadata.camera_model;
                self.metadata_user_comment = metadata.user_comment;
                self.metadata_gps_latitude = metadata.gps_latitude;
                self.metadata_gps_longitude = metadata.gps_longitude;
            }
            Message::OpenFiles => {
                self.path_of_all_images = load_files();

                let metadata = get_metadata(PathBuf::from(
                    "/home/mitch/Pictures/EXIFIR_TEST/000086280005.jpg",
                ));

                self.metadata_datetime_created = metadata.datetime_created;
                self.metadata_camera_brand = metadata.camera_brand;
                self.metadata_camera_model = metadata.camera_model;
                self.metadata_user_comment = metadata.user_comment;
                self.metadata_gps_latitude = metadata.gps_latitude;
                self.metadata_gps_longitude = metadata.gps_longitude;
            }
            Message::CloseFolder => {
                self.path_of_all_images = Vec::new();
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
        let open_folder_button = button("Open Folder").on_press(Message::OpenFolder);
        let open_multiple_files_button = button("Open Files").on_press(Message::OpenFiles);
        let reset_button = button("Close Current Folder").on_press(Message::CloseFolder);
        let menu = row![open_folder_button, open_multiple_files_button, reset_button].spacing(10);

        // IMAGE PANEL
        // let image_name: text::Text = text(&self.image_filename)
        //     .height(Shrink)
        //     .width(Fill)
        //     .height(20)
        //     .center();

        let mut image_grid = grid!().columns(3);

        if self.path_of_all_images.len() != 0 {
            for image in &self.path_of_all_images {
                let image: image::Image = image::Image::new(image).width(Fill).height(Fill);
                let image_name: text::Text = text("example-teehee.jpg")
                    .height(Shrink)
                    .width(Fill)
                    .height(20)
                    .center();
                let image_column = column!(image, image_name).spacing(10);
                let image_container = container(image_column)
                    .style(container::bordered_box)
                    .padding(5)
                    .max_width(300)
                    .max_height(300);
                image_grid = image_grid.push(image_container);
                self.path_of_all_images.iter().next();
            }
        } else {
            image_grid = image_grid.push(text(""));
        }

        let image_panel = scrollable(
            container(image_grid)
                .style(container::bordered_box)
                .height(Fill)
                .width(Fill)
                .center(Fill),
        );

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

fn load_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let file_handles = match FileDialog::new()
        .set_directory("Pictures/")
        .add_filter("images", &["jpg", "jpeg", "png"])
        .set_title("Choose multiple files...")
        .pick_files()
    {
        Some(file_handles) => file_handles,
        None => {
            println!("Could not get folder");
            Vec::new()
        }
    };

    for file in file_handles {
        files.push(
            file.as_path()
                .to_str()
                .expect("Failed to cast file path to string")
                .to_string(),
        );
    }

    files
}

fn load_folder() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let folder = match FileDialog::new()
        .set_directory("Pictures/")
        .set_title("Choose a folder...")
        .pick_folder()
    {
        Some(file_handle) => file_handle,
        None => {
            println!("Could not get folder");
            PathBuf::new()
        }
    };

    let folder_contents = fs::read_dir(folder).expect("OOOOPS");
    for file in folder_contents {
        files.push(
            file.expect("oops4")
                .path()
                .to_str()
                .expect("oops5")
                .to_string(),
        );
    }

    files
}

fn get_metadata(image_path: PathBuf) -> ImageMetadata {
    let mut image_metadata: ImageMetadata = ImageMetadata::new();

    let metadata: Metadata =
        Metadata::new_from_path(image_path.as_path()).expect("Failed to get metadata");

    let endian = metadata.get_endian();

    let datetime_created = match metadata
        .get_tag(&&ExifTag::DateTimeOriginal(String::new()))
        .next()
    {
        Some(datetime_created) => datetime_created,
        None => {
            println!("Could not get DateTimeOrigial from selected image");
            &ExifTag::DateTimeOriginal(String::new())
        }
    };

    image_metadata.datetime_created = String::from_u8_vec(
        &datetime_created.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let camera_brand = match metadata.get_tag(&&ExifTag::Make(String::new())).next() {
        Some(camera_brand) => camera_brand,
        None => {
            println!("Could not get Make from selected image");
            &ExifTag::Make(String::new())
        }
    };

    image_metadata.camera_brand = String::from_u8_vec(
        &camera_brand.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let camera_model = match metadata.get_tag(&&ExifTag::Model(String::new())).next() {
        Some(camera_model) => camera_model,
        None => {
            println!("Could not get Model from selected image");
            &ExifTag::Model(String::new())
        }
    };

    image_metadata.camera_model = String::from_u8_vec(
        &camera_model.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let user_comment = match metadata.get_tag(&&ExifTag::UserComment(Vec::new())).next() {
        Some(user_comment) => user_comment,
        None => {
            println!("Could not get UserComment from selected image");
            &ExifTag::UserComment(Vec::new())
        }
    };

    image_metadata.user_comment = String::from_u8_vec(
        &user_comment.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let gps_latitude = match metadata.get_tag(&&ExifTag::GPSLatitude(Vec::new())).next() {
        Some(gps_latitude) => gps_latitude,
        None => {
            println!("Could not get GPSLatitude from selected image");
            &ExifTag::GPSLatitude(Vec::new())
        }
    };

    image_metadata.gps_latitude = String::from_u8_vec(
        &gps_latitude.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    let gps_longitude = match metadata.get_tag(&&ExifTag::GPSLongitude(Vec::new())).next() {
        Some(gps_longitude) => gps_longitude,
        None => {
            println!("Could not get GPSLongitude from selected image");
            &ExifTag::GPSLatitude(Vec::new())
        }
    };

    image_metadata.gps_longitude = String::from_u8_vec(
        &gps_longitude.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    image_metadata
}

pub fn main() -> iced::Result {
    iced::application(Viewer::new, Viewer::update, Viewer::view)
        .title("Exifir")
        .run()
}
