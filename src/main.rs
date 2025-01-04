use clap::{Args, Parser, Subcommand};
use datatool::{
    AudioResampleArgs, AudioSplitArgs, AudioSummaryArgs, AudioTrimArgs, CountArgs, ImageResizeArgs,
    ImageSummaryArgs, ImageTessellateArgs, ImageToLandscapeArgs, ImageToPortraitArgs,
    PointcloudConvertArgs, PointcloudSummaryArgs, TableSchemaArgs, TableToCsvArgs,
    TableToParquetArgs, VideoSummaryArgs,
};

/// Rust implementation of bash commands
#[derive(Debug, Parser)]
#[clap(name = "datatool", version, author, about)]
pub struct RootArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Operations on audio data
    Audio(AudioCommand),
    /// Operations on image data
    Image(ImageCommand),
    /// Operations on video data
    Video(VideoCommand),
    /// Operations on filesystem files
    File(FileCommand),
    /// Operations on tabular data
    Table(TableCommand),
    /// Operations on pointcloud data
    Pointcloud(PointcloudCommand), // New command category
}

// ----------- AUDIO -----------
#[derive(Debug, Args)]
struct AudioCommand {
    #[clap(subcommand)]
    command: AudioSubCommand,
}

#[derive(Debug, Subcommand)]
enum AudioSubCommand {
    /// Get metadata about audio files (a single file or a directory).
    Summary(AudioSummaryArgs),
    /// Split audio files into chunks of specified duration.
    Split(AudioSplitArgs),
    /// Change the sample rate of audio files.
    Resample(AudioResampleArgs),
    /// Trim audio files to a specified length.
    Trim(AudioTrimArgs),
}

// ----------- IMAGE -----------
#[derive(Debug, Args)]
struct ImageCommand {
    #[clap(subcommand)]
    command: ImageSubCommand,
}

#[derive(Debug, Subcommand)]
enum ImageSubCommand {
    /// Get metadata about image files.
    Summary(ImageSummaryArgs),
    /// Resize images to specified dimensions.
    Resize(ImageResizeArgs),
    /// Split images into a grid of smaller images.
    Tessellate(ImageTessellateArgs),
    ToLandscape(ImageToLandscapeArgs),
    ToPortrait(ImageToPortraitArgs),
}

// ----------- VIDEO -----------
#[derive(Debug, Args)]
struct VideoCommand {
    #[clap(subcommand)]
    command: VideoSubCommand,
}

#[derive(Debug, Subcommand)]
enum VideoSubCommand {
    /// Get metadata about video files.
    Summary(VideoSummaryArgs),
}

// ----------- FILE -----------
#[derive(Debug, Args)]
struct FileCommand {
    #[clap(subcommand)]
    command: FileSubCommand,
}

#[derive(Debug, Subcommand)]
enum FileSubCommand {
    /// Count files and directories in a given path.
    Count(CountArgs),
}

// ----------- TABLE -----------
#[derive(Debug, Args)]
struct TableCommand {
    #[clap(subcommand)]
    command: TableSubCommand,
}

#[derive(Debug, Subcommand)]
enum TableSubCommand {
    /// Display the schema of a CSV or Parquet file.
    Schema(TableSchemaArgs),
    ToParquet(TableToParquetArgs),
    ToCsv(TableToCsvArgs),
}

// ----------- POINTCLOUD -----------
#[derive(Debug, Args)]
struct PointcloudCommand {
    #[clap(subcommand)]
    command: PointcloudSubCommand,
}

#[derive(Debug, Subcommand)]
enum PointcloudSubCommand {
    /// Summarize files in a given path.
    Summary(PointcloudSummaryArgs),
    /// Convert pointcloud file from one format to another.
    Convert(PointcloudConvertArgs),
}

fn main() {
    let args = RootArgs::parse();

    let result = match args.command {
        Command::Audio(audio_command) => match audio_command.command {
            AudioSubCommand::Summary(args) => datatool::commands::audio::summary::execute(args),
            AudioSubCommand::Split(args) => datatool::commands::audio::split::execute(args),
            AudioSubCommand::Resample(args) => datatool::commands::audio::resample::execute(args),
            AudioSubCommand::Trim(args) => datatool::commands::audio::trim::execute(args),
        },
        Command::Image(image_command) => match image_command.command {
            ImageSubCommand::Summary(args) => datatool::commands::image::summary::execute(args),
            ImageSubCommand::Resize(args) => datatool::commands::image::resize::execute(args),
            ImageSubCommand::Tessellate(args) => {
                datatool::commands::image::tessellate::execute(args)
            }
            ImageSubCommand::ToLandscape(args) => {
                datatool::commands::image::to_landscape::execute(args)
            }
            ImageSubCommand::ToPortrait(args) => {
                datatool::commands::image::to_portrait::execute(args)
            }
        },
        Command::Video(video_command) => match video_command.command {
            VideoSubCommand::Summary(args) => datatool::commands::video::summary::execute(args),
        },
        Command::File(file_command) => match file_command.command {
            FileSubCommand::Count(args) => datatool::commands::file::count::execute(args),
        },
        Command::Table(table_command) => match table_command.command {
            TableSubCommand::Schema(args) => datatool::commands::table::schema::execute(args),
            TableSubCommand::ToParquet(args) => {
                datatool::commands::table::to_parquet::execute(args)
            }
            TableSubCommand::ToCsv(args) => datatool::commands::table::to_csv::execute(args),
        },
        Command::Pointcloud(pointcloud_command) => match pointcloud_command.command {
            PointcloudSubCommand::Summary(args) => {
                datatool::commands::pointcloud::summary::execute(args)
            }
            PointcloudSubCommand::Convert(args) => {
                datatool::commands::pointcloud::convert::execute(args)
            }
        },
    };

    if let Err(e) = result {
        datatool::handle_error(e);
    }
}
