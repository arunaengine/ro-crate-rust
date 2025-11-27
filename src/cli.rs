use clap::{ArgAction, Parser, arg, command};
use clap_derive::Subcommand;
use std::error::Error;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short = 'c', long = "crate-path")]
    pub crate_dir: Option<String>,
}

/// Simple program to greet a person
#[derive(Debug, Subcommand)]
pub enum Commands {
    Add {
        #[command(subcommand)]
        command: AddCommand,

        #[arg(short = 'P', value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = ArgAction::Append)]
        property: Vec<(String, String)>,

        #[arg(short = 'p')]
        path: String,
    },
    Init {
        #[arg(long = "gen-preview")]
        gen_preview: bool,
        #[arg(short = 'e')]
        exclude: Option<String>,
    },
    WriteZip {
        #[arg(long = "dst")]
        destination: String,
    },
}

#[derive(Debug, Clone, Subcommand, Default)]
pub enum AddLanguage {
    #[default]
    Cwl,
    Galaxy,
    Knime,
    Nextflow,
    Snakemake,
    Compss,
    Autosubmit,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AddCommand {
    File,
    Dataset,
    TestDefinition {
        #[arg(long = "suite")]
        suite: String,
        #[arg(long = "definition-path")]
        definition_path: String,
        #[arg(short = 'e', long = "engine", default_value = "planemo")]
        engine: Option<String>,
        #[arg(short = 'v', long = "engine-version")]
        engine_version: Option<String>,
    },
    TestInstance {
        #[arg(long = "suite")]
        suite: String,
        #[arg(long = "url")]
        url: String,
        #[arg(short = 'r', long = "resource")]
        resource: Option<String>,
        #[arg(short = 's', long = "service", default_value = "jenkins")]
        service: Option<String>,
        #[arg(short = 'i', long = "identifier")]
        identifier: Option<String>,
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
    },
    TestSuite {
        #[arg(short = 'i', long = "identifier")]
        identifier: Option<String>,
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        #[arg(short = 'm', long = "main-entity")]
        main_entitiy: Option<String>,
    },
    Workflow {
        #[command(subcommand)]
        language: AddLanguage,
        #[arg(short = 'c')]
        crate_dir: Option<String>,
    },
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
