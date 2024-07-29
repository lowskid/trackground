use magick_rust::{MagickWand, magick_wand_genesis, PixelWand, bindings};
use mpris::{PlayerFinder, Metadata};
use std::sync::Once;

use std::process::Command;

use wallpaper;

use std::fs::File;
use std::io::copy;
use reqwest::blocking::get;

fn main() {
    let tempurl = "nil";
    while true {
        let player = PlayerFinder::new()
        .expect("Couldn't connect to D-Bus.")
        .find_active()
        .expect("Couldn't find a player.");

        let metadata = player.get_metadata();

        if let Some(coverurl) = metadata.expect("yep").art_url() {
        
            if tempurl != coverurl {
                println!("{}", coverurl);

                let mut response = get(coverurl).expect("Failed to download image");
                let mut file = File::create("bg").expect("Failed to create file");
                copy(&mut response, &mut file).expect("Failed to save image");
                                let mut response = get(coverurl).expect("Failed to download image");

                let mut file = File::create("fg").expect("Failed to create file");
                copy(&mut response, &mut file).expect("Failed to save image");

                let tempurl = coverurl;

                blur()
            }
        } else {
            println!("No URL found...");
        }
    }
}

fn blur() {
    let init = Once::new();
    init.call_once(|| {
        magick_wand_genesis();
    });

    let wand = MagickWand::new();

    wand.read_image("bg").unwrap();

    let sigma = 5.0;
    wand.gaussian_blur_image(0.0, sigma).unwrap();
   
    wand.write_image("bg").unwrap();

    let amount_str = "40";
    Command::new("convert")
        .arg("bg")
        .arg("-fill")
        .arg("black")
        .arg("-colorize")
        .arg(&amount_str)
        .arg("-resize")
        .arg("1900x1900")
        .arg("bg")
        .output()
        .expect("Failed to execute ImageMagick command");
    
    // Command::new("magick")
    //     .arg("fg")
    //     .arg("-background")
    //     .arg("none")
    //     .arg("-shadow")
    //     .arg("30x5+5+5")
    //     .arg("shadow.png")
    //     .output();
    //
    // Command::new("magick")
    //     .arg("shadow.png")
    //     .arg("fg")
    //     .arg("-compose")
    //     .arg("over")
    //     .arg("-composite")
    //     .arg("fg.png")
    //     .output();

    Command::new("convert")
        .arg("bg")
        .arg("fg")
        .arg("-gravity")
        .arg("Center")
        .arg("-geometry")
        .arg("300x300+0+0")
        .arg("-composite")
        .arg("tg")
        .output()
        .expect("Failed to execute ImageMagick command");

        wallpaper::set_from_path("tg");


}
