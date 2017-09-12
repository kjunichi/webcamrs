extern crate libc;
use self::libc::{c_char, c_int, c_void, c_uchar};

mod link;

pub enum Capture {}
pub enum IplImage {}

#[repr(C)]
pub struct CvMat {
    pub _type: c_int,
    pub step: c_int,

    pub refcount: *mut c_int,
    pub hdr_refcount: c_int,
    pub ptr: *mut c_uchar,
    pub rows: c_int,
    pub cols: c_int,
}

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
    pub fn cvEncodeImage(ext: *const c_char,
                         image: *mut IplImage,
                         params: *const c_int)
                         -> *mut CvMat;
}
