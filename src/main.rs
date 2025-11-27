use crate::cli::Cli;
use clap::Parser;
use ro_crate_rust::entity::EntityTrait;
use ro_crate_rust::{DataEntity, ROCrateError, read_rocrate};

mod cli;

fn main() -> Result<(), ROCrateError> {
    let Cli { command, crate_dir } = Cli::parse();

    let rocrate = match crate_dir {
        Some(dir) => read_rocrate(dir)?,
        None => read_rocrate("./")?,
    };

    match command {
        cli::Commands::Add {
            command,
            property,
            path,
        } => match command {
            cli::AddCommand::File => {
                let mut file = DataEntity::new(path);
                if !property.is_empty() {
                    for (key, value) in property {
                        file.set_property(key.into(), value.into());
                    }
                }
            }
            cli::AddCommand::Dataset => {
                let mut file = DataEntity::new_dataset(path);
                if !property.is_empty() {
                    for (key, value) in property {
                        file.set_property(key.into(), value.into());
                    }
                }
            }
            cli::AddCommand::TestDefinition {
                suite,
                definition_path,
                engine,
                engine_version,
            } => {
                todo!()
            }
            cli::AddCommand::TestInstance {
                suite,
                url,
                resource,
                service,
                identifier,
                name,
            } => todo!(),
            cli::AddCommand::TestSuite {
                identifier,
                name,
                main_entitiy,
            } => todo!(),
            cli::AddCommand::Workflow {
                language,
                crate_dir,
            } => todo!(),
        },
        cli::Commands::Init {
            gen_preview,
            exclude,
        } => todo!(),
        cli::Commands::WriteZip { destination } => todo!(),
    };

    Ok(())
}
