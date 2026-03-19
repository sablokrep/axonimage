use image::{self, imageops};
use std::error::Error;
use std::fs;

/*
Gaurav Sablok
codeprog@icloud.com
 */

pub fn smartcore_healthy(
    pathdir: &str,
    width: &str,
    height: &str,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let mut imagetensor: Vec<Vec<f32>> = Vec::new();
    for i in fs::read_dir(pathdir)? {
        let vecinter: Vec<Vec<u8>> = Vec::new();
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let imageopen = image::open(path_str).expect("file not present");
        let imagevec = imageopen
            .resize_exact(
                width.parse::<u32>().unwrap(),
                height.parse::<u32>().unwrap(),
                imageops::FilterType::Nearest,
            )
            .to_luma8()
            .pixels()
            .map(|x| x.0[0] as f32 / 255.0)
            .collect::<Vec<f32>>();
        imagetensor.push(imagevec);
    }
    Ok(imagetensor)
}

pub fn smartcore_diseased(
    pathdir: &str,
    width: &str,
    height: &str,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let mut imagetensor: Vec<Vec<f32>> = Vec::new();
    for i in fs::read_dir(pathdir)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let imageopen = image::open(path_str).expect("file not present");
        let imagevec = imageopen
            .resize_exact(
                width.parse::<u32>().unwrap(),
                height.parse::<u32>().unwrap(),
                imageops::FilterType::Nearest,
            )
            .to_luma8()
            .pixels()
            .map(|x| x.0[0] as f32 / 255.0)
            .collect::<Vec<f32>>();
        imagetensor.push(imagevec);
    }
    Ok(imagetensor)
}
