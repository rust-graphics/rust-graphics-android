use {
    super::activity,
    super::config,
    super::input,
    super::looper,
    super::rect,
    super::window,
    crate::libc,
    std::os::raw::{c_int, c_void},
};

#[cfg(feature = "verbose_log")]
use crate::log_i;

#[repr(C)]
pub struct AndroidPollSource {
    pub id: i32,
    pub app: &'static mut AndroidApp,
    pub process: extern "C" fn(app: &mut AndroidApp, source: &mut AndroidPollSource),
}

#[repr(C)]
pub struct AndroidApp {
    pub on_app_cmd: extern "C" fn(app: &mut AndroidApp, cmd: i32),
    pub on_input_event: extern "C" fn(app: &mut AndroidApp, event: *mut input::AInputEvent) -> i32,
    pub activity: &'static mut activity::ANativeActivity,
    pub config: &'static mut config::AConfiguration,
    pub saved_state: *mut c_void,
    pub saved_state_size: isize,
    pub looper: *mut looper::ALooper,
    pub input_queue: *mut input::AInputQueue,
    pub window: *mut window::ANativeWindow,
    pub content_rect: rect::ARect,
    pub activity_state: c_int,
    pub destroy_requested: c_int,
    pub mutex: libc::pthread_mutex_t,
    pub cond: libc::pthread_cond_t,
    pub msg_read_fd: c_int,
    pub msg_write_fd: c_int,
    pub thread: libc::pthread_t,
    pub cmd_poll_source: AndroidPollSource,
    pub input_poll_source: AndroidPollSource,
    pub running: c_int,
    pub state_saved: c_int,
    pub destroyed: c_int,
    pub redraw_needed: c_int,
    pub pending_input_queue: *mut input::AInputQueue,
    pub pending_window: *mut window::ANativeWindow,
    pub pending_content_rect: rect::ARect,
}

#[cfg(feature = "verbose_log")]
impl Drop for AndroidApp {
    fn drop(&mut self) {
        log_i!("AndroidApp droped.");
    }
}

#[repr(i32)]
pub enum LooperId {
    Main = 1,
    Input = 2,
    User = 3,
}

#[repr(i8)]
#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub enum AppCmd {
    InputChanged = 1,
    InitWindow = 2,
    TermWindow = 3,
    WindowResized = 4,
    WindowRedrawNeeded = 5,
    ContentRectChanged = 6,
    GainedFocus = 7,
    LostFocus = 8,
    ConfigChanged = 9,
    LowMemory = 10,
    Start = 11,
    Resume = 12,
    SaveState = 13,
    Pause = 14,
    Stop = 15,
    Destroy = 16,
    InternalError = -1,
}
