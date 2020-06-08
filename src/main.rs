extern crate rayon;

use rayon::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

pub type Complex64 = Complex<f64>;

impl<T> Complex<T> {
    #[inline]
    pub const fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl Complex64 {
    #[inline]
    pub fn add(&self, t: Complex64) -> Self {
        Self::new(
            self.re.clone() + t.re.clone(),
            self.im.clone() + t.im.clone(),
        )
    }

    #[inline]
    pub fn pow(&self) -> Complex64 {
        Complex64::new(
            self.re.clone() * self.re.clone() - self.im.clone() * self.im.clone(),
            2f64 * self.re.clone() * self.im.clone(),
        )
    }

    #[inline]
    pub fn value(&self) -> f64 {
        (self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()).sqrt()
    }
}

fn iter(c: Complex64, maxd: i32) -> i32 {
    let mut z = Complex64::new(0.0, 0.0);
    let mut i = 0;
    loop {
        let zn = z.pow().add(c);
        let zv = zn.value();
        if zv >= 2.0 || i >= maxd {
            break;
        }
        i = i + 1;
        z = zn;
    }
    return i;
}

pub fn compute_depth(width: i32, height: i32, maxd: i32, (x, y): (i32, i32)) -> i32 {
    let fx = 3.0 / width as f64;
    let fy = 2.0 / height as f64;
    let c = Complex::new(fx * x as f64 - 2.0, fy * y as f64 - 1.0);
    return iter(c, maxd);
}

fn main() {
    let height = 2000;
    let width = 3000;
    let maxd = 1000;

    let v: Vec<i32> = (0..width * height).collect();
    let w: Vec<i32> = v
        .par_iter()
        .map(|i| compute_depth(width, height, maxd, (i % height, i / height)))
        .collect();
    //    let w: Vec<i32> = v.iter().map(|i| compute_depth(width, height, maxd, (i % height, i / height))).collect();

    println!("Result size is {}.", w.len());
}
