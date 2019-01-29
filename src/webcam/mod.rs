pub mod ffi;

extern crate libc;

use std::ffi::CString;

pub struct Capture {
    raw: *mut ffi::Capture,
}

pub struct VideoCapture {
    raw: *mut ffi::CvVideoCapture
}

pub struct IplImage {
    raw: *mut ffi::IplImage,
}

pub struct ImgBuffer {
    raw: *mut ffi::ImgBuffer,
}

pub struct Mat {
    raw: *mut ffi::Cv2Mat,
    pub buf: Vec<u8>,
}

pub struct CvMat {
    raw: *mut ffi::CvMat,
    pub cols: i32,
    pub rows: i32,
    pub buf: Vec<u8>,
}

pub fn create_mat() -> Mat {
    Mat { buf: Vec::new(),
    raw: unsafe {ffi::cv_create_mat()}}
}

pub fn named_window(title: &str) -> () {
    unsafe {
        ffi::cv_named_window(CString::new(title).unwrap().as_ptr());
    }
}

pub fn read(capture: &VideoCapture, frame: &Mat) -> () {
    let raw_videocapture = capture.raw;
    let raw_mat = frame.raw;
    unsafe { ffi::cv_read(raw_videocapture, raw_mat); }
}

pub fn imwrite(filename: &str, frame: &Mat) -> i32 {
    let raw_mat = frame.raw;
    unsafe { ffi::cv_imwrite(CString::new(filename).unwrap().as_ptr(), raw_mat) }
}

pub fn mat_cols(img: &Mat) -> i32 {
    unsafe { ffi::cv_mat_cols(img.raw) }
}

pub fn mat_data(img: &Mat) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let len = unsafe { ffi::cv_mat_cols(img.raw) };
    let ptr = unsafe { ffi::cv_mat_data(img.raw) };
     for i in 0..len {
         buf.push(unsafe { *(ptr.offset(i as isize))});
     }
     return buf;
}

pub fn imencode(ext: &str, img: &Mat, params: Vec<i32>) -> Vec<u8>{
    let raw_mat = img.raw;
    let dst_buf = ImgBuffer { raw: unsafe { ffi::cv_imencode(CString::new(ext).unwrap().as_ptr(), raw_mat, 0 as *const i32) } };
    let mut buf: Vec<u8> = Vec::new();
    let ptr = unsafe { (*dst_buf.raw).ptr };
    let len = unsafe { (*dst_buf.raw).size };
    for i in 0..len {
        buf.push(unsafe { *(ptr.offset(i as isize)) });
    }
    unsafe { ffi::cv_free_img_buffer(dst_buf.raw) };
    return buf;
}

pub fn release(capture: &VideoCapture) -> () {
    let raw_videocapture = capture.raw;
    unsafe { ffi::cv_release_video_capture(raw_videocapture); }
}

pub fn video_capture(camnum: i32) -> VideoCapture {
    VideoCapture {
        raw: unsafe { ffi::cv_video_capture(camnum) }
    }
}

pub fn imshow(name: &str, image: &Mat) -> () {
    let raw_image = image.raw;
    unsafe {
        ffi::cv_imshow(CString::new(name).unwrap().as_ptr(), raw_image);
    }
}

pub fn wait_key(delay: i32) -> i32 {
    unsafe { ffi::cv_wait_key(delay) }
}

pub fn destroy_all_windows() ->() {
    unsafe {
        ffi::cv_destroy_all_windows();
    }
}

pub fn helloTest() -> () {
    unsafe {
        ffi::helloTest();
    }
}
