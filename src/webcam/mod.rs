pub mod ffi;

extern crate libc;

use std::ffi::CString;

pub struct Capture {
    raw: *mut ffi::Capture,
}

pub struct IplImage {
    raw: *mut ffi::IplImage,
}

pub struct CvMat {
    raw: *mut ffi::CvMat,
    pub cols: i32,
    pub rows: i32,
    pub buf: Vec<u8>,
}
pub fn destroy_all_windows() -> () {
    unsafe {
        ffi::cvDestroyAllWindows();
    }
}

pub fn named_window(title: &str) -> () {
    unsafe {
        ffi::cvNamedWindow(CString::new(title).unwrap().as_ptr());
    }
}

pub fn create_camera_capture(index: i32) -> Capture {
    Capture { raw: unsafe { ffi::cvCreateCameraCapture(index) } }
}

pub fn query_frame(capture: &Capture) -> IplImage {
    let raw_capture = capture.raw;

    IplImage { raw: unsafe { ffi::cvQueryFrame(raw_capture) } }
}

pub fn show_image(name: &str, image: &IplImage) -> () {
    let raw_image = image.raw;
    unsafe {
        ffi::cvShowImage(CString::new(name).unwrap().as_ptr(), raw_image);
    }
}

pub fn wait_key(delay: i32) -> i32 {
    unsafe { ffi::cvWaitKey(delay) }
}

pub fn save_image(name: &str, image: &IplImage) -> i32 {
    let raw_image = image.raw;
    unsafe {
        ffi::cvSaveImage(CString::new(name).unwrap().as_ptr(),
                         raw_image,
                         0 as *const i32)
    }
}

pub fn encode_image(ext: &str, image: &IplImage, params: Vec<i32>) -> CvMat {
    let raw_image = image.raw;
    let raw_mat = unsafe {
        ffi::cvEncodeImage(CString::new(ext).unwrap().as_ptr(),
                           raw_image,
                           0 as *const i32)
    };
    let len = unsafe { (*raw_mat).cols };
    let mut buf: Vec<u8> = Vec::new();
    for i in 0..len {
        let ptr = unsafe { (*raw_mat).ptr };
        buf.push(unsafe { *(ptr.offset(i as isize)) });
    }
    let mat = CvMat {
        raw: raw_mat,
        cols: unsafe { (*raw_mat).cols },
        rows: unsafe { (*raw_mat).rows },
        buf: buf,
    };
    return mat;
}

pub fn release_capture(capture: &Capture) -> () {
    let raw_capture = capture.raw;
    unsafe {
        ffi::cvReleaseCapture(&raw_capture);
    }
}
