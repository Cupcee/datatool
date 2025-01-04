use clap::{Args, Parser};

pub mod commands;
pub mod utils;

// Export all the Args structs as they're needed by both the CLI and tests
#[derive(Debug, Parser)]
pub struct CountArgs {
    /// Target directory or file
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct ImageSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct ImageResizeArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Requested height
    #[arg(required = true)]
    pub height: u32,

    /// Requested width
    #[arg(required = true)]
    pub width: u32,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Flag to enable overwriting of input file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct ImageTessellateArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Number of vertical patches
    #[arg(required = true)]
    pub n_vertical: u32,

    /// Number of horizontal patches
    #[arg(required = true)]
    pub n_horizontal: u32,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Delete original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub delete_original: bool,
}

#[derive(Debug, Parser)]
pub struct ImageToLandscapeArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Delete original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct ImageToPortraitArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Delete original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct AudioSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    pub target: String,
}

#[derive(Debug, Parser)]
pub struct AudioSplitArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Chunk duration in seconds
    #[arg(required = true)]
    pub chunk_duration: f32,

    /// Output directory
    #[arg(required = true)]
    pub output: String,

    /// Delete original file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub delete_original: bool,
}

#[derive(Debug, Parser)]
pub struct AudioResampleArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Target sample rate
    #[arg(required = true)]
    pub sr: u32,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Flag to enable overwriting of input file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct AudioTrimArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Target length in seconds
    #[arg(required = true)]
    pub length: f32,

    /// Output file or directory
    #[arg(required = true)]
    pub output: String,

    /// Start offset in seconds
    #[arg(default_value_t = 0.0)]
    pub offset: f32,

    /// Flag to enable overwriting of input file
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub overwrite: bool,
}

#[derive(Debug, Parser)]
pub struct VideoSummaryArgs {
    /// Target directory or file
    #[arg(required = true)]
    pub target: String,
}

#[derive(Debug, Args)]
pub struct TableSchemaArgs {
    /// Input file (CSV or parquet)
    #[arg(required = true)]
    pub input: String,
}

#[derive(Debug, Args)]
pub struct TableToParquetArgs {
    /// Input CSV file or directory
    #[arg(required = true)]
    pub input: String,

    /// Output directory or file
    #[arg(required = true)]
    pub output: String,
}

#[derive(Debug, Args)]
pub struct TableToCsvArgs {
    /// Input parquet file
    #[arg(required = true)]
    pub input: String,

    /// Output directory or file
    #[arg(required = true)]
    pub output: String,
}

#[derive(Debug, Args)]
pub struct PointcloudSummaryArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// If provided, recursively process directories
    #[clap(long)]
    pub recursive: bool,

    /// Strict PCD schema. If provided, the schema is expected to be precisely x, y, z,
    /// rgb where all are F32. Otherwise, we only try dynamically parsing xyz information.
    #[clap(long)]
    pub strict_pcd_schema: bool,
}

#[derive(Debug, Args)]
pub struct PointcloudConvertArgs {
    /// Input pointcloud file (e.g. .las, .laz, .pcd)
    #[arg(required = true)]
    pub input: String,
    /// Output pointcloud file (e.g. .las, .pcd)
    #[arg(required = true)]
    pub output: String,

    /// Optional output format hint (not needed if output extension is known)
    #[clap(long)]
    pub format: Option<String>,

    /// Strict PCD schema. If provided, the schema is expected to be precisely x, y, z,
    /// rgb where all are F32. Otherwise, we only try dynamically parsing xyz information.
    #[clap(long)]
    pub strict_pcd_schema: bool,
}

#[derive(Debug, Args)]
pub struct PointcloudHarmonizeArgs {
    /// Input file or directory
    #[arg(required = true)]
    pub input: String,

    /// Output pointcloud directory
    #[arg(required = true)]
    pub output: String,

    /// If provided, recursively process directories
    #[clap(long)]
    pub recursive: bool,
}

// Error handling utility that can be used by both lib and binary
pub fn handle_error(e: anyhow::Error) {
    eprintln!("Error!");
    for (i, cause) in e.chain().enumerate() {
        eprintln!("  Cause {}: {}", i, cause);
    }
    std::process::exit(1);
}
