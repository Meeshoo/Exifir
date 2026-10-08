use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use iced::widget::{
    Column, button, column, container, grid, image, row, scrollable, space, text, text_input,
};
use iced::{Fill, Shrink};
use little_exif::u8conversion::U8conversion;
use rfd::FileDialog;

struct Viewer {
    path_of_all_images: Vec<PathBuf>,
    selected_image: PathBuf,
    selected_image_datetime_created: String,
    selected_image_camera_make: String,
    selected_image_camera_model: String,
    selected_image_user_comment: String,
    selected_image_description: String,
    // selected_image_gps_latitude: String,
    // selected_image_gps_longitude: String,
}

struct ImageMetadata {
    image_path: String,
    image_filename: String,
    datetime_created: String,
    camera_make: String,
    camera_model: String,
    user_comment: String,
    description: String,
    // gps_latitude: String,
    // gps_longitude: String,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFolder,
    OpenFiles,
    CloseFolder,
    SelectImage { file_path: PathBuf },
    LoadMetadata,
    SaveMetadeta,
    DateTimeCreatedChanged(String),
    CameraMakeChanged(String),
    CameraModelChanged(String),
    UserCommentChanged(String),
    ImageDescriptionChanged(String),
}

// #[derive(Debug, Clone)]
// enum Error {
//     DialogClosed,
//     MetadataGetFailed,
// }

impl ImageMetadata {
    fn new() -> Self {
        ImageMetadata {
            image_path: String::new(),
            image_filename: String::new(),
            datetime_created: String::new(),
            camera_make: String::new(),
            camera_model: String::new(),
            user_comment: String::new(),
            description: String::new(),
            // gps_latitude: String::new(),
            // gps_longitude: String::new(),
        }
    }
}

impl Viewer {
    fn new() -> Self {
        Viewer {
            path_of_all_images: Vec::new(),
            selected_image: PathBuf::new(),
            selected_image_datetime_created: String::from(""),
            selected_image_camera_make: String::from(""),
            selected_image_camera_model: String::from(""),
            selected_image_user_comment: String::from(""),
            selected_image_description: String::from(""),
            // selected_image_gps_latitude: String::from(""),
            // selected_image_gps_longitude: String::from(""),
        }
    }

    // fn theme(&self) -> Theme {
    //     Theme::Light
    // }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFolder => {
                self.path_of_all_images = load_folder();
            }
            Message::OpenFiles => {
                self.path_of_all_images = load_files();
            }
            Message::CloseFolder => {
                self.path_of_all_images = Vec::new();
                self.selected_image_datetime_created = String::from("");
                self.selected_image_camera_make = String::from("");
                self.selected_image_camera_model = String::from("");
                self.selected_image_user_comment = String::from("");
                self.selected_image_description = String::from("");
                // self.selected_image_gps_latitude = String::from("");
                // self.selected_image_gps_longitude = String::from("");
            }
            Message::SelectImage { file_path } => {
                self.selected_image = file_path;
            }
            Message::LoadMetadata => {
                let metadata = get_metadata(self.selected_image.to_owned());
                self.selected_image_datetime_created = metadata.datetime_created;
                self.selected_image_camera_make = metadata.camera_make;
                self.selected_image_camera_model = metadata.camera_model;
                self.selected_image_user_comment = metadata.user_comment;
                self.selected_image_description = metadata.description;
                // self.selected_image_gps_latitude = metadata.gps_latitude;
                // self.selected_image_gps_longitude = metadata.gps_longitude;
            }
            Message::SaveMetadeta => {
                save_metadata(
                    &self.selected_image.clone(),
                    &self.selected_image_datetime_created,
                    &self.selected_image_camera_make,
                    &self.selected_image_camera_model,
                    &self.selected_image_user_comment,
                    &self.selected_image_description,
                    // &self.selected_image_gps_latitude,
                    // &self.selected_image_gps_longitude,
                );
            }
            Message::DateTimeCreatedChanged(datetime_original) => {
                self.selected_image_datetime_created = datetime_original;
            }
            Message::CameraMakeChanged(camera_make) => {
                self.selected_image_camera_make = camera_make;
            }
            Message::CameraModelChanged(camera_model) => {
                self.selected_image_camera_model = camera_model;
            }
            Message::UserCommentChanged(user_comment) => {
                self.selected_image_user_comment = user_comment;
            }
            Message::ImageDescriptionChanged(image_description) => {
                self.selected_image_description = image_description;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        // MENU
        let open_folder_button = button("Open Folder").on_press(Message::OpenFolder);
        let open_multiple_files_button = button("Open Files").on_press(Message::OpenFiles);
        let load_metadata_button =
            button("Load Metadata from Selected Image").on_press(Message::LoadMetadata);
        let save_metadata_button =
            button("Save Metadata to Selected Image").on_press(Message::SaveMetadeta);
        let reset_button = button("Close Current Folder").on_press(Message::CloseFolder);

        let menu = row![
            open_folder_button,
            open_multiple_files_button,
            space::Space::new().width(Fill),
            load_metadata_button,
            save_metadata_button,
            space::Space::new().width(Fill),
            reset_button
        ]
        .spacing(10);

        // IMAGE PANEL
        let mut image_grid = grid!().columns(5);

        if self.path_of_all_images.len() != 0 {
            for item in &self.path_of_all_images {
                let image: image::Image =
                    image::Image::new(item.as_path()).width(Fill).height(Fill);
                let image_name: text::Text = text(
                    item.file_name()
                        .expect("Failed to get filename")
                        .to_str()
                        .expect("Failed to get str from filename")
                        .to_string(),
                )
                .height(Shrink)
                .width(Fill)
                .center();
                let image_column = column!(image, image_name).spacing(10);
                let image_container;
                if &self.selected_image == item {
                    image_container = button(
                        container(image_column)
                            .style(container::bordered_box)
                            .padding(5),
                    )
                    .on_press(Message::SelectImage {
                        file_path: item.into(),
                    })
                    .style(button::primary);
                } else {
                    image_container = button(
                        container(image_column)
                            .style(container::bordered_box)
                            .padding(5),
                    )
                    .on_press(Message::SelectImage {
                        file_path: item.into(),
                    })
                    .style(button::subtle);
                }
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
        let image_datetime_label = text("Datetime Original: ");
        let selected_image_datetime_created = text_input("", &self.selected_image_datetime_created)
            .on_input(Message::DateTimeCreatedChanged);

        let image_camera_make = text("Camera Make: ");
        let selected_image_camera_make =
            text_input("", &self.selected_image_camera_make).on_input(Message::CameraMakeChanged);

        let image_camera_model = text("Camera Model: ");
        let selected_image_camera_model =
            text_input("", &self.selected_image_camera_model).on_input(Message::CameraModelChanged);

        let image_user_comment = text("User Comment: ");
        let selected_image_user_comment =
            text_input("", &self.selected_image_user_comment).on_input(Message::UserCommentChanged);

        let image_description = text("Image Description: ");
        let selected_image_description = text_input("", &self.selected_image_description)
            .on_input(Message::ImageDescriptionChanged);

        // let selected_image_gps_latitude: text::Text =
        //     text("GPS Latitude: ".to_owned() + &self.selected_image_gps_latitude)
        //         .height(Shrink)
        //         .width(Shrink)
        //         .center();

        // let selected_image_gps_longitude: text::Text =
        //     text("GPS Longitude: ".to_owned() + &self.selected_image_gps_longitude)
        //         .height(Shrink)
        //         .width(Shrink)
        //         .center();

        let metadata_container: Column<'_, Message> = column!(
            image_datetime_label,
            selected_image_datetime_created,
            image_camera_make,
            selected_image_camera_make,
            image_camera_model,
            selected_image_camera_model,
            image_user_comment,
            selected_image_user_comment,
            image_description,
            selected_image_description,
            // selected_image_gps_latitude,
            // selected_image_gps_longitude
        )
        .width(350)
        .spacing(10)
        .padding(50);

        let metadata_panel: container::Container<'_, _> = container(metadata_container)
            .style(container::bordered_box)
            .height(Fill)
            .width(350)
            .center(Shrink);

        let main_workspace = row!(image_panel, metadata_panel);

        let interface = column!(menu, main_workspace).padding(10).spacing(10);
        interface
    }
}

fn load_files() -> Vec<PathBuf> {
    let file_handles = match FileDialog::new()
        .set_directory("Pictures/")
        .add_filter(
            "images",
            &["jpg", "JPG", "jpeg", "JPEG", "png", "PNG", "tiff", "TIFF"],
        )
        .set_title("Choose multiple files...")
        .pick_files()
    {
        Some(file_handles) => file_handles,
        None => {
            println!("Could not get folder");
            Vec::new()
        }
    };

    file_handles
}

fn load_folder() -> Vec<PathBuf> {
    let valid_file_extensions: Vec<&OsStr> = vec![
        OsStr::new("jpg"),
        OsStr::new("JPG"),
        OsStr::new("jpeg"),
        OsStr::new("JPEG"),
        OsStr::new("png"),
        OsStr::new("PNG"),
        OsStr::new("tiff"),
        OsStr::new("TIFF"),
    ];
    let mut files: Vec<PathBuf> = Vec::new();
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
        if file
            .as_ref()
            .expect("Failed to get file")
            .file_type()
            .expect("Failed to get file type")
            .is_dir()
            || !valid_file_extensions.contains(
                &file
                    .as_ref()
                    .expect("Failed to get file")
                    .path()
                    .extension()
                    .expect("Failed to get file extension"),
            )
        {
            continue;
        }
        files.push(file.expect("Failed to get file").path());
    }

    files
}

fn get_metadata(image_path: PathBuf) -> ImageMetadata {
    let mut image_metadata: ImageMetadata = ImageMetadata::new();

    image_metadata.image_path = image_path
        .to_str()
        .expect("Failed to convert image path to string")
        .to_string();

    image_metadata.image_filename = image_path
        .file_name()
        .expect("Failed to get filename from path")
        .to_str()
        .expect("Failed to convert filename to str")
        .to_string();

    let metadata: Metadata = match Metadata::new_from_path(image_path.as_path()) {
        Ok(metadata) => metadata,
        Err(error) => {
            println!("{error}");
            return image_metadata;
        }
    };

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

    let camera_make = match metadata.get_tag(&&ExifTag::Make(String::new())).next() {
        Some(camera_make) => camera_make,
        None => {
            println!("Could not get Make from selected image");
            &ExifTag::Make(String::new())
        }
    };

    image_metadata.camera_make = String::from_u8_vec(
        &camera_make.value_as_u8_vec(&metadata.get_endian()),
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

    let description = match metadata
        .get_tag(&&ExifTag::ImageDescription(String::new()))
        .next()
    {
        Some(description) => description,
        None => {
            println!("Could not get ImageDescription from selected image");
            &ExifTag::ImageDescription(String::new())
        }
    };

    image_metadata.description = String::from_u8_vec(
        &description.value_as_u8_vec(&metadata.get_endian()),
        &endian,
    );

    // let gps_latitude = match metadata.get_tag(&&ExifTag::GPSLatitude(Vec::new())).next() {
    //     Some(gps_latitude) => gps_latitude,
    //     None => {
    //         println!("Could not get GPSLatitude from selected image");
    //         &ExifTag::GPSLatitude(Vec::new())
    //     }
    // };

    // image_metadata.gps_latitude = String::from_u8_vec(
    //     &gps_latitude.value_as_u8_vec(&metadata.get_endian()),
    //     &endian,
    // );

    // let gps_longitude = match metadata.get_tag(&&ExifTag::GPSLongitude(Vec::new())).next() {
    //     Some(gps_longitude) => gps_longitude,
    //     None => {
    //         println!("Could not get GPSLongitude from selected image");
    //         &ExifTag::GPSLatitude(Vec::new())
    //     }
    // };

    // image_metadata.gps_longitude = String::from_u8_vec(
    //     &gps_longitude.value_as_u8_vec(&metadata.get_endian()),
    //     &endian,
    // );

    image_metadata
}

fn save_metadata(
    image_path: &PathBuf,
    datetime_created: &str,
    camera_make: &str,
    camera_model: &str,
    user_comment: &str,
    description: &str,
    // gps_latitude: &str,
    // gps_longitude: &str,
) {
    let mut metadata: Metadata = match Metadata::new_from_path(image_path.as_path()) {
        Ok(metadata) => metadata,
        Err(error) => {
            println!("{error}");
            Metadata::new()
        }
    };

    metadata.set_tag(ExifTag::DateTimeOriginal(datetime_created.into()));
    metadata.set_tag(ExifTag::Make(camera_make.into()));
    metadata.set_tag(ExifTag::Model(camera_model.into()));
    metadata.set_tag(ExifTag::UserComment(user_comment.into()));
    metadata.set_tag(ExifTag::ImageDescription(description.into()));
    // metadata.set_tag(ExifTag::GPSLatitude(gps_latitude));
    // metadata.set_tag(ExifTag::GPSLongitude(gps_longitude));

    let _ = metadata.write_to_file(image_path.as_path());
}

pub fn main() -> iced::Result {
    iced::application(Viewer::new, Viewer::update, Viewer::view)
        .title("Exifir")
        .run()
}
