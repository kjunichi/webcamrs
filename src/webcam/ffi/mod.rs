extern crate libc;
// use self::libc::{ c_char, c_double, c_int, c_schar, c_void }
use self::libc::{c_char, c_int, c_void};
// use std::ffi::CString;

pub enum Capture {}
pub enum IplImage {}

#[cfg(windows)]
#[link(name = "opencv_world320")]
extern "C" {
    pub fn cvDestroyAllWindows() -> c_void;
    pub fn cvNamedWindow(title: *const c_char) -> c_int;
    pub fn cvShowImage(name: *const c_char, image: *const IplImage) -> c_void;
    pub fn cvWaitKey(delay: c_int) -> c_int;
    pub fn cvCreateCameraCapture(index: c_int) -> *mut Capture;
    pub fn cvQueryFrame(capture: *mut Capture) -> *mut IplImage;
    pub fn cvSaveImage(filename: *const c_char,
                       image: *const IplImage,
                       params: *const c_int)
                       -> c_int;
    pub fn cvReleaseCapture(capture: &*mut Capture) -> c_void;
}

#[cfg(not(windows))]
#[link(name = "opencv_highgui")]
#[link(name = "opencv_videoio")]
#[link(name = "opencv_imgcodecs")]
extern "C" {
    pub fn cvDestroyAllWindows() -> c_void;
    pub fn cvNamedWindow(title: *const c_char) -> c_int;
    pub fn cvShowImage(name: *const c_char, image: *const IplImage) -> c_void;
    pub fn cvWaitKey(delay: c_int) -> c_int;
    pub fn cvCreateCameraCapture(index: c_int) -> *mut Capture;
    pub fn cvQueryFrame(capture: *mut Capture) -> *mut IplImage;
    pub fn cvSaveImage(filename: *const c_char,
                       image: *const IplImage,
                       params: *const c_int)
                       -> c_int;
    pub fn cvReleaseCapture(capture: &*mut Capture) -> c_void;
}
