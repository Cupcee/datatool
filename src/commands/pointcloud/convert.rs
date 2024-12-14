use anyhow::{bail, Result};

use pasture_core::containers::VectorBuffer;
use pasture_io::base::PointWriter;
use pasture_io::las::LASWriter;
use pasture_io::las_rs::{Builder, Transform, Vector as LasVector, Version};

use crate::commands::pointcloud::pointcloud_utils::{
    compute_bounds, extension, is_supported_extension, read_pointcloud_file_to_buffer,
};
use crate::PointcloudConvertArgs;

pub fn execute(args: PointcloudConvertArgs) -> Result<()> {
    let input_ext = extension(&args.input);
    let output_ext = extension(&args.output);

    if !is_supported_extension(&input_ext) {
        bail!("Unsupported input format: {}", input_ext);
    }
    if !is_supported_extension(&output_ext) {
        bail!("Unsupported output format: {}", output_ext);
    }

    let buffer = read_pointcloud_file_to_buffer(&args.input)?;

    // Currently only supports writing LAS/LAZ output
    if output_ext == "las" || output_ext == "laz" {
        write_las_file(&args.output, &buffer)?;
        println!("Converted '{}' to '{}'", args.input, args.output);
    } else {
        bail!("Currently only supports writing to LAS/LAZ output")
    }

    Ok(())
}

/// Write VectorBuffer to a LAS/LAZ file.
fn write_las_file(path: &str, buffer: &VectorBuffer) -> Result<()> {
    // Create a LAS header with some default values.
    let mut builder = Builder::from(Version::new(1, 4));
    builder.generating_software = "datatool".to_string();
    builder.system_identifier = "datatool".to_string();

    let (min_x, _max_x, min_y, _max_y, min_z, _max_z) = compute_bounds(buffer);

    let scale = 0.001; // example scale
    let x_transform = Transform {
        scale,
        offset: min_x,
    };
    let y_transform = Transform {
        scale,
        offset: min_y,
    };
    let z_transform = Transform {
        scale,
        offset: min_z,
    };
    builder.transforms = LasVector {
        x: x_transform,
        y: y_transform,
        z: z_transform,
    };

    let header = builder.into_header()?;
    let mut writer = LASWriter::from_path_and_header(path, header)?;

    writer.write(buffer)?;
    writer.flush()?;
    Ok(())
}
