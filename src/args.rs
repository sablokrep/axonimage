use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "axonimage",
    version = "1.0",
    about = "Entire image modelling in AxonML
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// image datasets
    Convolutional {
        /// path to the folder with the coral normal images
        normal: String,
        /// path to the folder with the coral bleached images
        bleached: String,
        /// thread for the analysis
        thread: String,
    },
    /// RandomForest
    RandomForest {
        /// path for the folder for the coral normal images
        normal: String,
        /// path for the folder with the coral bleached images
        bleached: String,
        /// width of the samples
        width: String,
        /// height of the samples
        height: String,
        /// threads for the analysis
        thread: String,
    },
}
