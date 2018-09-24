extern crate libc;
use self::libc::{c_char, c_int, c_void, c_uchar};

mod link;

pub enum Capture {}
pub enum IplImage {}
pub enum CvVideoCapture {}
pub enum Cv2Mat {}

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
    pub fn cv_destroy_all_windows() -> c_void;
    pub fn cv_named_window(title: *const c_char) -> c_void;
    //pub fn cv_read(capture: *mut CvVideoCapture, frame: *mut Cv2Mat);
    pub fn cv_read(capture: *mut CvVideoCapture) -> *mut Cv2Mat;
    pub fn cv_imshow(winname: *const c_char, mat: *const Cv2Mat) -> c_void;
    pub fn cv_video_capture(camnum: c_int) -> *mut CvVideoCapture;
    pub fn cv_wait_key(delay: c_int) -> c_int;
    pub fn cv_release_video_capture(capture: *mut CvVideoCapture) -> c_void;
    pub fn cv_create_mat() -> *mut Cv2Mat;

    pub fn cvNamedWindow(title: *const c_char) -> c_int;
    pub fn cvShowImage(name: *const c_char, image: *const IplImage) -> c_void;
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
#[link(name = "webcam", kind = "static")]
extern "C" {
    pub fn helloTest() -> c_void;
}
