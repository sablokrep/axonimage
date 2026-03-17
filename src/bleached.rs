use crate::Error;
use crate::rotation::arrayfilldiseased;
use crate::rotation::rgbcost;
use std::fs;

type VECTYPE = (Vec<Vec<f32>>, Vec<f32>);

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn makevecdiseased(pathfile: &str) -> Result<VECTYPE, Box<dyn Error>> {
    let mut vecimagefinal: Vec<Vec<u8>> = Vec::new();
    let veclabels = arrayfilldiseased(&vecimagefinal)
        .iter()
        .map(|x| x.to_string().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    for i in fs::read_dir(pathfile.to_string())? {
        let mut vecinter: Vec<Vec<u8>> = Vec::new();
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let imgread = image::open(path_str).unwrap().to_rgb8();
        for (_x, _y, pixel) in imgread.enumerate_pixels() {
            let r = pixel.0[0];
            let g = pixel.0[1];
            let b = pixel.0[2];
            let mut vecimage: Vec<u8> = Vec::new();
            vecimage.push(r);
            vecimage.push(g);
            vecimage.push(b);
            vecinter.push(vecimage);
        }
        let finalvecinter = vecinter.into_iter().flatten().collect::<Vec<u8>>();
        vecimagefinal.push(finalvecinter);
    }

    let cannoical_image = vecimagefinal
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| x.to_string().parse::<f32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((cannoical_image, veclabels))
}

/*
Making a collatable tensor for the diseased ones
*/

pub fn makevecdiseasedcummulative(pathfile: &str) -> Result<VECTYPE, Box<dyn Error>> {
    let mut vecimagefinal: Vec<Vec<u8>> = Vec::new();
    let veclabels = arrayfilldiseased(&vecimagefinal)
        .iter()
        .map(|x| x.to_string().parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    for i in fs::read_dir(pathfile.to_string())? {
        let mut vecinter: Vec<Vec<u8>> = Vec::new();
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let imgread = image::open(path_str).unwrap().to_rgb8();
        for (_x, _y, pixel) in imgread.enumerate_pixels() {
            let r = pixel.0[0];
            let g = pixel.0[1];
            let b = pixel.0[2];
            let mut vecimage: Vec<u8> = Vec::new();
            vecimage.push(r);
            vecimage.push(g);
            vecimage.push(b);
            vecinter.push(vecimage);
        }
        let finalvecinter = vecinter.into_iter().flatten().collect::<Vec<u8>>();
        vecimagefinal.push(finalvecinter);
    }

    let cannonical = vecimagefinal
        .iter()
        .map(|x| {
            x.iter()
                .map(|a| a.to_string().parse::<f32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok((rgbcost(&cannonical.clone()), veclabels))
}
