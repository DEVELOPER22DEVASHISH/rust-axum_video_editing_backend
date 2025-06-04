use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use ffprobe::ffprobe;
use std::io;


pub async fn trim_video(
    input_path: &str,
    output_path: &str,
    start_time: &str,
    duration: &str,
) -> Result<(), io::Error> {
    FfmpegBuilder::new()
        .option(Parameter::Single("-ss"))
        .option(Parameter::Single(start_time))
        .option(Parameter::Single("-t"))
        .option(Parameter::Single(duration))
        .input(File::new(input_path))
        .output(
            File::new(output_path)
                .option(Parameter::Single("-c"))
                .option(Parameter::Single("copy"))
        )
        .run()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

/// Overlay text subtitles on a video
pub async fn add_subtitles(
    input_path: &str,
    output_path: &str,
    subtitle_text: &str,
    start_time: &str,
    end_time: &str,
) -> Result<(), io::Error> {
    let start_num: f64 = start_time.parse()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid start_time"))?;
    let end_num: f64 = end_time.parse()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid end_time"))?;

    if end_num <= start_num {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid start/end times"));
    }

    let safe_text = subtitle_text.replace('\'', "\\'");
    let filter = format!(
        "drawtext=text='{}':enable='between(t,{},{})':fontcolor=white:fontsize=24:x=(w-text_w)/2:y=h-50",
        safe_text, start_num, end_num
    );

    FfmpegBuilder::new()
        .input(File::new(input_path))
        .output(
            File::new(output_path)
                .option(Parameter::Single("-vf"))
                .option(Parameter::Single(&filter))
        )
        .run()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

pub async fn get_video_duration_in_seconds(file_path: &str) -> Result<f64, io::Error> {
    let metadata = ffprobe(file_path).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("ffprobe failed: {}", e))
    })?;

    if let Some(duration_str) = metadata.format.duration {
        return duration_str
            .parse::<f64>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Parse error: {}", e)));
    }

    for stream in metadata.streams {
        if stream.codec_type == Some("video".to_string()) {
            if let Some(duration_str) = stream.duration {
                return duration_str
                    .parse::<f64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Parse error: {}", e)));
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Duration not found"))
}


// #[derive(Debug, Deserialize)]
// struct FFprobeFormat {
//     duration: Option<f64>,
// }

// #[derive(Debug, Deserialize)]
// struct FFprobeStream {
//     codec_type: String,
//     duration: Option<f64>,
// }

// #[derive(Debug, Deserialize)]
// struct FFprobeResult {
//     format: FFprobeFormat,
//     streams: Vec<FFprobeStream>,
// }


// Probe a video file and return its duration in seconds.
// pub async fn get_video_duration_in_seconds(file_path: &str) -> Result<f64, io::Error> {
//     let output = ffprober::new()
//         .input(file_path)
//         .show_format()
//         .show_streams()
//         .json()
//         .run()
//         .await?;

//     let ffprobe_result: FFprobeResult = serde_json::from_slice(&output.stdout)
//         .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to parse ffprobe output: {}", e)))?;

//     if let Some(duration) = ffprobe_result.format.duration {
//         return Ok(duration);
//     }
//     if let Some(stream) = ffprobe_result
//         .streams
//         .iter()
//         .find(|s| s.codec_type == "video" && s.duration.is_some())
//     {
//         return Ok(stream.duration.unwrap());
//     }
//     Err(io::Error::new(io::ErrorKind::Other, "Could not determine video duration"))
// }
