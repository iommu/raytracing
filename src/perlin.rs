use crate::{
    utils::random_int_range,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut randvec: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];
        let mut perm_x: [i32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_y: [i32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_z: [i32; POINT_COUNT] = [0; POINT_COUNT];

        // todo : map
        for index in 0..POINT_COUNT {
            randvec[index] = Vec3::random_from_range(-1.0, 1.0).unit_vector();
        }

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    #[allow(dead_code)]
    pub fn noise(&self, point: Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i32 as usize;
        let j = point.y().floor() as i32 as usize;
        let k = point.z().floor() as i32 as usize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[(self.perm_x[(i + di) & 0xFF]
                        ^ self.perm_y[(j + dj) & 0xFF]
                        ^ self.perm_z[(k + dk) & 0xFF])
                        as usize];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    #[allow(dead_code)]
    pub fn turb(&self, mut point: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.0;
        }

        accum.abs()
    }

    #[allow(dead_code)]
    fn perlin_generate_perm(arr: &mut [i32; POINT_COUNT]) {
        for index in 0..POINT_COUNT {
            arr[index] = index as i32;
        }

        Self::permute(arr, POINT_COUNT);
    }

    #[allow(dead_code)]
    fn permute(arr: &mut [i32; POINT_COUNT], n: usize) {
        for index in (1..n).rev() {
            let target = random_int_range(0, index as i32) as usize;
            let tmp = arr[index];
            arr[index] = arr[target];
            arr[target] = tmp;
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = di as f64;
                    let j = dj as f64;
                    let k = dk as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += c[di][dj][dk].dot(&weight_v)
                        * (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww));
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
