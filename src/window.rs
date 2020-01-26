use {super::rect::ARect, std::os::raw::c_void};

pub type ANativeWindow = c_void;

#[repr(u32)]
pub enum WindowFormat {
    Rgba8888 = 1,
    Rgbx8888 = 2,
    Rgb565 = 4,
}

#[repr(C)]
pub struct ANativeWindowBuffer {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub format: i32,
    pub bits: *mut c_void,
    pub reserved: [u32; 6usize],
}

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern "C" {
    pub fn ANativeWindow_acquire(window: *mut ANativeWindow);
    pub fn ANativeWindow_release(window: *mut ANativeWindow);
    pub fn ANativeWindow_getWidth(window: *mut ANativeWindow) -> i32;
    pub fn ANativeWindow_getHeight(window: *mut ANativeWindow) -> i32;
    pub fn ANativeWindow_getFormat(window: *mut ANativeWindow) -> i32;
    pub fn ANativeWindow_setBuffersGeometry(
        window: *mut ANativeWindow,
        width: i32,
        height: i32,
        format: i32,
    ) -> i32;
    pub fn ANativeWindow_lock(
        window: *mut ANativeWindow,
        out_buffer: *mut ANativeWindowBuffer,
        in_out_dirty_bounds: *mut ARect,
    ) -> i32;
    pub fn ANativeWindow_unlockAndPost(window: *mut ANativeWindow) -> i32;
}
