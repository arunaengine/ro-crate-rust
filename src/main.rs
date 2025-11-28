use crate::cli::Cli;
use clap::Parser;
use ro_crate_rust::entity::EntityTrait;
use ro_crate_rust::{
    DataEntity, ROCrate, ROCrateBuilder, ROCrateError, read_rocrate, write_rocrate,
};
use std::path::Path;

mod cli;

fn main() -> Result<(), ROCrateError> {
    let Cli { command, crate_dir } = Cli::parse();

    let getcrate = || -> Result<ROCrate, ROCrateError> {
        match &crate_dir {
            Some(dir) => read_rocrate(dir),
            None => read_rocrate("./"),
        }
    };

    match command {
        cli::Commands::Init {
            gen_preview,
            exclude,
        } => {
            let mut builder = ROCrateBuilder::new();

            let path = match &crate_dir {
                Some(dir) => Path::new(dir),
                None => Path::new("./"),
            };

            let to_exclude = match &exclude {
                Some(list) => list.split(";").collect(),
                None => vec![],
            };
            for entry in path.read_dir().expect("ead_dir call failed") {
                if let Ok(entry) = entry {
                    if to_exclude.contains(&entry.path().to_str().unwrap()) {
                        continue;
                    }
                    let metadata = entry.metadata()?;
                    if metadata.is_dir() {
                        builder = builder
                            .add_dataset(entry.path().to_str().map(|p| p.to_string()).unwrap())
                            .finish();
                    }
                    if metadata.is_file() {
                        let name = entry.path().to_str().map(|p| p.to_string()).unwrap();
                        let split = name.rsplit_once('.').unwrap().1;
                        let encoding = get_type(split);

                        println!("{} {}", split, encoding);

                        builder = builder
                            .add_file(name.clone())
                            .with_encoding_format(encoding)
                            .finish();
                    }
                }
            }

            if gen_preview {
                dbg!("TML previews are not implemented yet");
            }

            let rocrate = builder.build()?;
            write_rocrate(&rocrate, path)?;
        }
        cli::Commands::WriteZip { destination } => {
            let rocrate = getcrate()?;
            write_rocrate(&rocrate, format!("{destination}.zip"))?;
        }
        cli::Commands::Add {
            command,
            property,
            path,
        } => match command {
            cli::AddCommand::File => {
                let mut rocrate = getcrate()?;
                let mut file = DataEntity::new(path);
                if !property.is_empty() {
                    for (key, value) in property {
                        file.set_property(key.into(), value.into());
                    }
                }
                rocrate.add_data_entity(file);
                write_rocrate(&rocrate, crate_dir.unwrap_or("./".to_string()))?;
            }
            cli::AddCommand::Dataset => {
                let mut rocrate = getcrate()?;
                let mut file = DataEntity::new_dataset(path);
                if !property.is_empty() {
                    for (key, value) in property {
                        file.set_property(key.into(), value.into());
                    }
                }
                rocrate.add_data_entity(file);
                write_rocrate(&rocrate, crate_dir.unwrap_or("./".to_string()))?;
            }
        },
    };

    Ok(())
}

fn get_type(ending: &str) -> String {
    match ending {
        "aac" => "audio/aac",
        "abw" => "application/x-abiword",
        "apng" => "image/apng",
        "arc" => "application/x-freearc",
        "avif" => "image/avif",
        "avi" => "video/x-msvideo",
        "azw" => "application/vnd.amazon.ebook",
        "bin" => "application/octet-stream",
        "bmp" => "image/bmp",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "cda" => "application/x-cdf",
        "csh" => "application/x-csh",
        "css" => "text/css",
        "csv" => "text/csv",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application/vnd.ms-fontobject",
        "epub" => "application/epub+zip",
        "gz" => "application/x-gzip.",
        "gif" => "image/gif",
        "htm" | "html" => "text/html",
        "ico" => "image/vnd.microsoft.icon",
        "ics" => "text/calendar",
        "jar" => "application/java-archive",
        "jpeg" | "jpg" => "image/jpeg",
        "js" => "text/javascript (Specifications: HTML and RFC 9239)",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "md" => "text/markdown",
        "mid" | "midi" => "audio/x-midi",
        "mjs" => "text/javascript",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "mpkg" => "application/vnd.apple.installer+xml",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "ogx" => "application/ogg",
        "opus" => "audio/ogg",
        "otf" => "font/otf",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "rar" => "application/vnd.rar",
        "rtf" => "application/rtf",
        "sh" => "application/x-sh",
        "svg" => "image/svg+xml",
        "tar" => "application/x-tar",
        "tif" | "tiff" => "image/tiff",
        "ts" => "video/mp2t",
        "ttf" => "font/ttf",
        "txt" => "text/plain",
        "vsd" => "application/vnd.visio",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webmanifest" => "application/manifest+json",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application/xml serves as a valid default.",
        "xul" => "application/vnd.mozilla.xul+xml",
        "zip" => "application/x-zip-compressed.",
        "3gp" => "audio/3gpp if it doesn't contain video",
        "3g2" => "audio/3gpp2 if it doesn't contain video",
        "7z" => "application/x-7z-compressed",
        _ => "ext/plain", // or application/octet-stream?
    }
    .to_string()
}
