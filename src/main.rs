extern crate image;
#[macro_use(s)]
extern crate ndarray;
extern crate num;
extern crate rustdct;
#[warn(unused_imports)]
use image::{DynamicImage, GenericImageView};
use ndarray::{arr2, Array, Array2};
use rustdct::algorithm::type2and3_butterflies::Type2And3Butterfly8;
use rustdct::DCTplanner;
use rustdct::{DCT2, DCT3};
use std::ops::Sub;
use std::path::Path;
use std::sync::Arc;

fn dct_mat(size: u64) -> Array2<f64> {
    let N = size as f64;
    let mut dct_mat: Vec<f64> = vec![];
    let pi = std::f64::consts::FRAC_1_PI;
    let c_k = |n| {
        if n == 0 {
            (1f64 / 2f64).sqrt()
        } else {
            1f64
        }
    };
    for i in 0..size as u64 {
        for j in 0..size as u64 {
            dct_mat.push(
                (2.0 / N).sqrt() * c_k(i) * (((i as f64 - 1.0) * (j as f64 - 0.5) * pi) / N).cos(),
            )
        }
    }

    Array::from_vec(dct_mat)
        .into_shape((size as usize, size as usize))
        .unwrap()
}

fn read_pic(fname: String) -> Array2<f64> {
    let img = image::open(&Path::new(&fname)).expect("cant open image");
    let (h, w) = img.dimensions();
    let gray_pix = img.grayscale().raw_pixels();
    let f64_pix: Vec<f64> = gray_pix.into_iter().map(|x| x as f64).collect();

    let mat = Array::from_vec(f64_pix.to_vec())
        .into_shape((h as usize, w as usize))
        .unwrap();
    mat
}

fn trans_each_block(mut src_mat: Array2<f64>, trans_mat: Array2<f64>) -> Vec<Array2<f64>> {
    let mut last: i32 = 0;
    let mut range: Vec<i32> = vec![];
    let n = trans_mat.nrows();
    let h = src_mat.nrows();
    let w = src_mat.ncols();
    let mut transed_mat: Vec<Array2<f64>> = vec![];

    for i in 0..h / n {
        range.push(((i + 1) * n) as i32);
        let block = src_mat
            .slice_mut(s![last..range[i as usize], last..range[i as usize]])
            .dot(&trans_mat);
        transed_mat.push(trans_mat.t().dot(&block.slice(s![.., ..])));
        last = range[i as usize];
    }
    transed_mat
}

pub fn main() {
    let mut mat = read_pic("./lenna256.bmp".to_string());
    let h = mat.rows();
    let w = mat.cols();
    println!("{:?}", mat);
    let mut last: i32 = 0;
    let mut range: Vec<i32> = vec![];
    let dct_mat = dct_mat(8);
    let c = trans_each_block(mat, dct_mat);

    println!("{:?}", c);
}
