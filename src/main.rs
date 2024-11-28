use imageproc::contours::find_contours;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::error::Error;
use std::fs::{self, DirEntry};
use std::path::Path;
use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use image::io::Reader as ImageReader;
use image::ImageBuffer;
use image::Luma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <image_path_or_dir> <output_path_or_dir>", args[0]);
        return Err("Not enough arguments provided".into());
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let output_is_dir = Path::new(output_path).is_dir();

    if Path::new(input_path).is_dir() {
        let entries = fs::read_dir(input_path)?;
        for entry in entries {
            let entry = entry?;
            if is_image_file(&entry) {
                process_image(entry.path().to_str().unwrap().to_string(), output_path.to_string(), true).await?;
            }
        }
    } else {
        process_image(input_path.to_string(), output_path.to_string(), output_is_dir).await?;
    }

    Ok(())
}

async fn process_image(image_path: String, output_path: String, is_dir: bool) -> Result<(), Box<dyn Error>> {
    let image_path_clone = image_path.clone();

    let img: ImageBuffer<Luma<u8>, Vec<u8>> = tokio::task::spawn_blocking(move || {
        ImageReader::open(&image_path_clone).expect("Failed to open image")
            .decode().expect("Failed to decode image")
            .to_luma8()
    }).await.expect("Task join error");

    let threshold = 128;
    let binarized_img = imageproc::contrast::threshold(&img, threshold);
    let contours = find_contours::<u32>(&binarized_img);

    let pb = ProgressBar::new(contours.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, eta {eta})")
            .progress_chars("#>-"),
    );

    let mut data = Data::new();
    for contour in contours.into_iter() {
        pb.inc(1);

        if let Some(start_point) = contour.points.get(0) {
            data = data.move_to((start_point.x as i32, start_point.y as i32));
            for point in contour.points.iter().skip(1) {
                data = data.line_to((point.x as i32, point.y as i32));
            }
            data = data.close();
        }
    }

    pb.finish_with_message("完了!");

    let path = SvgPath::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, img.width(), img.height()))
        .add(path);

    let output_file_path = if is_dir {
        format!("{}/{}.svg", output_path, Path::new(&image_path).file_stem().unwrap().to_str().unwrap())
    } else {
        output_path
    };

    let mut file = File::create(output_file_path).await?;
    let doc_str = document.to_string();
    file.write_all(doc_str.as_bytes()).await?;
    file.flush().await?;

    Ok(())
}

fn is_image_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        match ext.to_str().unwrap().to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "bmp" | "gif" => true,
            _ => false,
        }
    } else {
        false
    }
}
