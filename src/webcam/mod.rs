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

pub struct Mat {
    raw: *mut ffi::Cv2Mat,
}
pub struct CvMat {
    raw: *mut ffi::CvMat,
    pub cols: i32,
    pub rows: i32,
    pub buf: Vec<u8>,
}

pub fn create_mat() -> Mat {
    Mat { raw: unsafe {ffi::cv_create_mat()}}
}

pub fn named_window(title: &str) -> () {
    unsafe {
        ffi::cv_named_window(CString::new(title).unwrap().as_ptr());
    }
}

pub fn create_camera_capture(index: i32) -> Capture {
    Capture { raw: unsafe { ffi::cvCreateCameraCapture(index) } }
}

pub fn query_frame(capture: &Capture) -> IplImage {
    let raw_capture = capture.raw;

    IplImage { raw: unsafe { ffi::cvQueryFrame(raw_capture) } }
}

// pub fn readBad(capture: &VideoCapture, frame: &Mat) -> () {
//     let raw_videocapture = capture.raw;
//     let raw_mat = frame.raw;
//     unsafe { ffi::cv_read(raw_videocapture, raw_mat) }
// }

pub fn read(capture: &VideoCapture) -> Mat {
    let raw_videocapture = capture.raw;
    Mat { raw: unsafe { ffi::cv_read(raw_videocapture) } }
}

pub fn show_image(name: &str, image: &IplImage) -> () {
    let raw_image = image.raw;
    unsafe {
        ffi::cvShowImage(CString::new(name).unwrap().as_ptr(), raw_image);
    }
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
    let ptr = unsafe { (*raw_mat).ptr };
    for i in 0..len {
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
