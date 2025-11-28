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
            gen_preview: _,
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
            for entry in path.read_dir().expect("read_dir call failed") {
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
                        builder = builder
                            .add_file(entry.path().to_str().map(|p| p.to_string()).unwrap())
                            .finish();
                    }
                }
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
