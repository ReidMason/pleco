use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PlecoArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // List common files
    ListCommon(ListCommonFiles),
    // Count occurances
    Count(Count),
    // Pull files
    PullFiles(PullFiles),
}

#[derive(Debug, Args)]
pub struct PullFiles {
    // Filepath to the first directory
    #[clap(default_value("."))]
    pub filepath: String,

    // Output directory
    #[clap(default_value("./output"))]
    pub output_dir: String,
}

#[derive(Debug, Args)]
pub struct ListCommonFiles {
    // Filepath to the first directory
    #[clap(default_value("."))]
    pub filepath: String,
}

#[derive(Debug, Args)]
pub struct Count {
    // Filename or directory name to search for
    pub search: String,

    // Filepath to the first directory
    #[clap(default_value("."))]
    pub filepath: String,
}
