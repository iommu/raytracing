use std::{
    alloc::{Layout, alloc, dealloc},
    env,
    ffi::CString,
    io::{self, Error},
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
};

use stb_image::stb_image::stbi_loadf;

use crate::vec3::Color;

#[repr(C)]
struct CBuffer<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> CBuffer<T> {
    fn new(len: usize) -> Self {
        assert!(len > 0);
        let layout = Layout::array::<T>(len).expect("Invalid length");
        let ptr = unsafe { alloc(layout) as *mut T };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Self { ptr, len }
    }

    fn from_ptr(ptr: *mut T, len: usize) -> Self {
        assert!(len > 0);
        Self { ptr, len }
    }

    fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T> Drop for CBuffer<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() || self.len == 0 {
            return;
        }

        unsafe {
            let layout = Layout::array::<T>(self.len).expect("Invalid length");
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> IndexMut<usize> for CBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "index out of bounds");
        unsafe { &mut *self.ptr.add(index) }
    }
}

impl<T> Index<usize> for CBuffer<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds!"); // Bounds check
        unsafe { &*self.ptr.add(index) }
    }
}

#[derive(Default)]
pub struct RTWImage {
    fdata: Option<CBuffer<f32>>, // Linear floating point pixel data
    bdata: Option<CBuffer<u8>>,  // Linear 8-bit pixel data
    image_width: i32,            // Loaded image width
    image_height: i32,           // Loaded image height
    bytes_per_scanline: i32,
}

impl RTWImage {
    const BYTES_PER_PIXEL: i32 = 3;

    #[allow(dead_code)]
    pub fn new(image_name: &str) -> io::Result<Self> {
        // Loads image data from the specified file. If the RTW_IMAGES environment variable is
        // defined, looks only in that directory for the image file. If the image was not found,
        // searches for the specified image file first from the current directory, then in the
        // images/ subdirectory, then the _parent's_ images/ subdirectory, and then _that_
        // parent, on so on, for six levels up. If the image was not loaded successfully,
        // width() and height() will return 0.

        let image_dir = env::var("RTW_IMAGES").unwrap_or_default();
        let mut rtw = Self::default();
        if !image_dir.is_empty() && rtw.load(format!("{image_dir}/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../../images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../../../images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../../../../images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../../../../../images/{image_name}")) {
            return Ok(rtw);
        }
        if rtw.load(format!("../../../../../../images/{image_name}")) {
            return Ok(rtw);
        }
        Err(Error::new(io::ErrorKind::Other, "Couldn't find image"))
    }

    #[allow(dead_code)]
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> bool {
        let mut n = Self::BYTES_PER_PIXEL;
        let path = CString::from_str(path.as_ref().as_os_str().to_str().unwrap_or_default())
            .unwrap_or_default();

        let ptr = unsafe {
            stbi_loadf(
                path.as_ptr(),
                &mut self.image_width,
                &mut self.image_height,
                &mut n,
                Self::BYTES_PER_PIXEL,
            )
        };

        if ptr.is_null() {
            return false;
        }
        let len = self.image_width * self.image_height * n;
        self.fdata = Some(CBuffer::from_ptr(ptr, len as usize));

        self.bytes_per_scanline = self.image_width * Self::BYTES_PER_PIXEL;
        self.convert_to_bytes();

        true
    }

    #[allow(dead_code)]
    pub fn width(&self) -> i32 {
        self.image_width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> i32 {
        self.image_height
    }

    #[allow(dead_code)]
    pub fn pixel_at(&self, mut x: i32, mut y: i32) -> [u8; 3] {
        // Return the address of the three RGB bytes of the pixel at x,y. If there is no image
        // data, returns magenta.
        if self.bdata.is_some() || self.bdata.as_ref().unwrap().is_null() {
            Color::new(255.0, 0.0, 255.0);
        }

        x = Self::clamp(x, 0, self.image_width);
        y = Self::clamp(y, 0, self.image_height);

        let offset = y * self.bytes_per_scanline + x * Self::BYTES_PER_PIXEL;
        let offset_max =
            self.image_height * self.bytes_per_scanline + self.image_width * Self::BYTES_PER_PIXEL;

        let offset = Self::clamp(offset, 0, offset_max - 3) as usize;
        self.bdata.as_ref().unwrap().as_slice()[offset..offset + 3]
            .try_into()
            .expect("Slice length not met")
    }

    fn convert_to_bytes(&mut self) {
        let fdata = self.fdata.as_mut().unwrap();

        // Convert the linear floating point pixel data to bytes, storing the resulting byte
        // data in the `bdata` member.

        let total_bytes = fdata.len;
        let mut bdata = CBuffer::<u8>::new(total_bytes as usize);

        // Iterate through all pixel components, converting from [0.0, 1.0] float values to
        // unsigned [0, 255] byte values.

        for index in 0..total_bytes {
            bdata[index] = Self::float_to_byte(fdata[index]);
        }
        self.bdata = Some(bdata);
    }

    #[allow(dead_code)]
    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        // Return the value clamped to the range [low, high).
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        return high - 1;
    }

    #[allow(dead_code)]
    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            return 0;
        }
        if 1.0 <= value {
            return 255;
        }
        return (256.0 * value) as u8;
    }
}
