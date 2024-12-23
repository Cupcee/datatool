# datatool

A Swiss-army knife CLI tool for data inspection and manipulation, written in Rust.

__Note:__ Credit to original author/repo [rush](https://github.com/giacomopiccinini/rush)
for creating the base structure. This fork plans to add additional data types
as I encounter a need for them. Currently Pointcloud input is added on top.

## Installation

```bash
cargo install --path .
```

Be aware that some extra dependecies are needed, mostly related to FFMpeg. On Debian-like
systems, ensure you run first

```bash
sudo apt update && apt install -y ffmpeg libavformat-dev libavutil-dev \
    libavcodec-dev libavfilter-dev libavdevice-dev libclang-dev
```

If the standard cargo installation fails, please consider using the provided
Dockerfile. To build that run

```bash
docker build --tag datatool .
```

Mounting a volume for containerized version:

```bash
docker run -v "$(pwd)/test-directory:/app/test-directory" datatool <COMMAND> /app/test-directory
```

## Command Categories

### Audio Commands

#### `audio summary`

Get metadata about audio files (a single file or a directory).

__Supported Extensions__: `.mp3`, `.wav`, `.ogg`, `.flac`, `.aac`, `.m4a`  
__Input__: Can be a single file or directory (recursive)

```bash
datatool audio summary <target>
```

Example:

```bash
datatool audio summary music/
```

Output:

```txt
Total files: 42
Total Duration: 02:15:30
Average Duration: 193 s
Sample Rates: {44100, 48000} Hz
Channels: {1, 2}
Bit Depths: {16, 24}
Unique durations: 42
Min duration: 120.5 s
Max duration: 345.2 s
```

#### `audio split`

Split audio files into chunks of specified duration.

__Supported Extensions__ `.wav` only  
__Input__ Can be a single file or directory (recursive)

```bash
datatool audio split <input> <chunk_duration> <output> [--delete-original]
```

Example:

```bash
datatool audio split long.wav 30 chunks/ --delete-original
```

This will split long.wav into 30-second chunks and save them in the `chunks/`
directory. The original `long.wav` file will be deleted.

#### `audio resample`

Change the sample rate of audio files.

__Supported Extensions__ `.wav` only  
__Input__ Can be a single file or directory (recursive)

```bash
datatool audio resample <input> <sr> <output> [--overwrite]
```

Example:

```bash
datatool audio resample input.wav 44100 output.wav
```

#### `audio trim`

Trim audio files to a specified length.

__Supported Extensions__ `.wav` only  
__Input__ Can be a single file or directory (recursive)

```bash
datatool audio trim <input> <length> <output> [--offset <seconds>] [--overwrite]
```

Example:

```bash
datatool audio trim input.wav 60 output.wav --offset 30
```

### Image Commands

#### `image summary`

Get metadata about image files.

__Supported Extensions__ `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
__Input__ Can be a single file or directory (recursive)

```bash
datatool image summary <target>
```

Example:

```bash
datatool image summary photos/
```

Output:

```txt
Total files: 156
Unique (height, width) pairs: {(1080, 1920), (800, 600), (3024, 4032)}
```

#### `image resize`

Resize images to specified dimensions.

__Supported Extensions__ `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
__Input__ Can be a single file or directory (recursive)

```bash
datatool image resize <input> <height> <width> <output> [--overwrite]
```

Example:

```bash
datatool image resize input.jpg 1080 1920 output.jpg
```

#### `image tessellate`

Split images into a grid of smaller images.

__Supported Extensions__ `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
__Input__ Can be a single file or directory (recursive)

```bash
datatool image tessellate <input> <n_vertical> <n_horizontal> <output> [--delete-original]
```

Example:

```bash
datatool image tessellate photo.jpg 2 3 tiles/
```

This splits the image into a 2×3 grid (6 pieces).

### Video Commands

#### `video summary`

Get metadata about video files.

__Supported Extensions__ `.ts`, `.mp4`, `.mkv`, `.mov`  
__Input__ Can be a single file or directory (recursive)

```bash
datatool video summary <target>
```

Example:

```bash
datatool video summary videos/
```

Output:

```txt
Total files: 12
Total duration: 3600.5
Unique durations: {120, 240, 360}
Unique (height, width) pairs: {(1080, 1920), (720, 1280)}
Unique FPS: {(30, 1), (60, 1)}
```

### File Commands

#### `file count`

Count files and directories in a given path.

__Supported Extensions__ All files  
__Input__ Can be a single file or directory (non-recursive, only immediate children)

```bash
datatool file count <target>
```

Example:

```bash
datatool file count documents/
```

Output:

```txt
Files: 145
Directories: 12
```

### Table Commands

#### `table schema`

Display the schema of a CSV or Parquet file.

__Supported Extensions__ `.csv`, `.parquet`  
__Input__ Single file only (directories not supported)

```bash
datatool table schema <input>
```

Example:

```bash
datatool table schema data.csv
```

Output:

```txt
Schema:
name: Name, field: String
name: Age, field: Int64
name: Department, field: String
name: Salary, field: Int64
```

### Pointcloud commands

#### `pointcloud summary`

Summarize files in a given path.

__Supported Extensions__ `.pcd`, `.las`, `.laz`
__Input__ Can be a single file or directory (non-recursive, only immediate children)

```bash
datatool pointcloud summary [--dynamic-pcd-schema] <target>
```

Output:

```txt
Number of files: 3
Total number of points: 639
Bounding box:
  X: [-0.9552599787712097, 0.9918500185012817]
  Y: [-0.3154599964618683, 0.3564099967479706]
  Z: [0, 0]
```

#### `pointcloud convert`

Convert pointcloud file from one format to another.

__Supported inputs__ `.pcd` (single file)
__Supported outputs__ `.laz`, `.las` (single file)

```bash
datatool pointcloud convert [--dynamic-pcd-schema] <input-file-path> <output-file-path>
```
