use std::process;
use image::{self, FilterType, GenericImageView};
use std::path::Path;

const REQUIRE_ARGS_CNT: usize = 3;
const PATH_INDEX: usize = 1;
const SIZE_INDEX: usize = 2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != REQUIRE_ARGS_CNT {
        println!("Not found {} arguments! Please, order arguments count: {}", REQUIRE_ARGS_CNT - 1, args.len());
        process::exit(1);
    }
    let path = Path::new(&args[PATH_INDEX]);
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    let ext = path.extension().unwrap().to_string_lossy().to_lowercase();
    if ext.ne("jpg") && ext.ne("jpeg") {
        println!("{}\n", "Not jpeg file");
    }
    let &size = &args[SIZE_INDEX].parse::<usize>().unwrap();

    let (_resized_data, _, _) = match resize(path, size) {
        Ok(v) => v,
        Err(_e) => process::exit(1),
    };

    let attached = path.join(".resized");
    println!("{}", attached.display());
    println!("{}", filename);
}

fn resize(path: &Path, target_size: usize) -> Result<(Vec<u8>, usize, usize), String> {
    let img = image::open(path).map_err(|e| e.to_string())?;
    let width = img.width() as usize;
    let height = img.height() as usize;

    if width > target_size || height > target_size {
        let (target_width, target_height) =
            if width > height {
                let ratio: f32 = target_size as f32 / width as f32;
                (target_size, (height as f32 * ratio) as usize)
            } else {
                let ratio: f32 = target_size as f32 / height as f32;
                ((width as f32 * ratio) as usize, target_size)
            };
        let resized = img.resize(
            target_width as u32,
            target_height as u32,
            FilterType::Lanczos3,
        );
        Ok((resized.to_rgb().to_vec(), resized.width() as usize, resized.height() as usize))
    } else {
        Ok((img.to_rgb().to_vec(), width, height))
    }
}
