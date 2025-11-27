use clap::{ArgAction, Parser, arg, command};
use clap_derive::Subcommand;
use std::error::Error;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Simple program to greet a person
#[derive(Debug, Subcommand)]
pub enum Commands {
    Add {
        #[command(subcommand)]
        command: AddCommand,
    },

    Init {
        gen_preview: bool,
        exclude: Option<String>,
        create_dir: Option<String>,
    },
    Write {
        #[command(subcommand)]
        command: WriteCommand,
    },
}
#[derive(Debug, Clone, Subcommand)]
pub enum WriteCommand {
    Dataset,
    File,
    TestDefinition,
    TestInstance,
    TestSuite,
    Workflow,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AddLanguage {
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
    Dataset,
    File,
    TestDefinition,
    TestInstance,
    TestSuite,
    Workflow {
        #[command(subcommand)]
        language: AddLanguage,
        #[arg(short = 'c')]
        create_dir: Option<String>,
        #[arg(short = 'P', value_name = "KEY=VALUE", value_parser = parse_key_val::<String, String>, action = ArgAction::Append)]
        property: Vec<(String, String)>,
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
