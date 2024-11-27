use image::GenericImageView;
use imageproc::contours::find_contours;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::error::Error;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <image_path> <output_path>", args[0]);
        return Err("Not enough arguments provided".into());
    }

    let image_path = &args[1];
    let output_path = &args[2];
    let img = image::open(image_path)?.to_luma8();
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

    let path = Path::new()
        .set("fill", "black")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, img.width(), img.height()))
        .add(path);

    svg::save(output_path, &document)?;

    Ok(())
}
