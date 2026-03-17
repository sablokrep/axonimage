use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn normalizevec(
    a: Vec<Vec<f32>>,
    b: Vec<f32>,
    c: Vec<Vec<f32>>,
    d: Vec<f32>,
) -> Result<(Vec<Vec<f32>>, Vec<f32>), Box<dyn Error>> {
    let mut finala: Vec<Vec<f32>> = Vec::new();
    let mut finalb: Vec<f32> = Vec::new();
    for i in 0..a.len() {
        finala.push(a[i].clone());
    }
    for i in 0..c.len() {
        finala.push(c[i].clone());
    }
    for i in 0..b.len() {
        finalb.push(b[i].clone());
    }
    for i in 0..d.len() {
        finalb.push(d[i].clone());
    }
    Ok((finala, finalb))
}
