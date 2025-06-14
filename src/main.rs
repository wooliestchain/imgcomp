use image::{GenericImageView, Pixel};
use std::env;
use std::process;

fn mse(img1 : &image::DynamicImage, img2: &image::DynamicImage) -> f64 {
    let (w, h) = img1.dimensions();
    assert_eq!((w,h), img2.dimensions(), "Les images doivent avoir les mêmes tailles");

    let mut  sum = 0f64;
    for y in 0..h{
        for x in 0..w{
            let p1 = img1.get_pixel(x, y).to_rgb();
            let p2 = img2.get_pixel(x, y).to_rgb();

            for i in 0..3{
                let diff = p1[i] as f64 - p2[i] as f64;
                sum += diff * diff;
            }
        }
    }

    sum / ((w * h * 3) as f64)
}

fn psnr(mse: f64, max_pixel: f64) -> f64 {
    10.0 * ((max_pixel * max_pixel) / mse).log10()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <image1> <image2>", args[0]);
        process::exit(1);
    }

    let img1 = image::open(&args[1]).expect("Impossible d'ouvrir l'image 1");
    let img2 = image::open(&args[2]).expect("Impossible d'ouvrir l'image 2");

    let mse_val = mse(&img1, &img2);
    let psnr_val = psnr(mse_val, 255.0);

    println!("MSE : {:.2}", mse_val);
    println!("PSNR : {:.2} dB", psnr_val);
}
