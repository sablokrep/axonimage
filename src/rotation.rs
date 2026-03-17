/*
Gaurav Sablok
codeprog@icloud.com
 */

pub fn arrayfillhealthy(arrayvector: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut arraynew: Vec<usize> = Vec::new();
    let arraylength = arrayvector.len();
    let mut i = 0i32;
    while i < arraylength as i32 {
        arraynew.push(0usize);
        i += 1i32;
    }
    arraynew
}

pub fn arrayfilldiseased(arrayvector: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut arraynew: Vec<usize> = Vec::new();
    let arraylength = arrayvector.len();
    let mut i = 0i32;
    while i < arraylength as i32 {
        arraynew.push(1usize);
        i += 1i32;
    }
    arraynew
}

pub fn rgbcost(arrayvect: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    // sum up all the RGB values of the images as they are not a boundary box
    let mut newvec: Vec<Vec<f32>> = Vec::new();
    for i in 0..arrayvect.len() {
        let mut rvec: Vec<_> = Vec::new();
        let mut gvec: Vec<f32> = Vec::new();
        let mut bvec: Vec<f32> = Vec::new();
        rvec.push(arrayvect[i][0]);
        gvec.push(arrayvect[i][1]);
        bvec.push(arrayvect[i][2]);
        let mut finalvec: Vec<f32> = Vec::new();
        let finalr = rvec.iter().sum();
        let finalg = gvec.iter().sum();
        let finalb = bvec.iter().sum();
        finalvec.push(finalr);
        finalvec.push(finalg);
        finalvec.push(finalb);
        newvec.push(finalvec);
    }
    newvec
}
