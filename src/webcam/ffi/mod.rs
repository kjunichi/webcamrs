extern crate libc;
use self::libc::{c_char, c_int, c_void, c_uchar};

mod link;

pub enum Capture {}
pub enum IplImage {}
pub enum CvVideoCapture {}
pub enum Cv2Mat {}

#[repr(C)]
pub struct ImgBuffer {
    pub ptr: *mut c_uchar,
    pub size: c_int,
    pub raw: *mut c_void,
}

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
    pub fn cv_read(capture: *mut CvVideoCapture, frame: *mut Cv2Mat) -> c_void;
    //pub fn cv_read(capture: *mut CvVideoCapture) -> *mut Cv2Mat;
    pub fn cv_imshow(winname: *const c_char, mat: *const Cv2Mat) -> c_void;
    pub fn cv_video_capture(camnum: c_int) -> *mut CvVideoCapture;
    pub fn cv_wait_key(delay: c_int) -> c_int;
    pub fn cv_release_video_capture(capture: *mut CvVideoCapture) -> c_void;
    pub fn cv_create_mat() -> *mut Cv2Mat;
    pub fn cv_imwrite(filename: *const c_char, mat: *const Cv2Mat) -> c_int;
    // pub fn cv_imencode(ext: *const c_char, img: *const Cv2Mat, 
    //                     params: *const c_int) -> *mut Cv2Mat;
    pub fn cv_mat_cols(mat: *const Cv2Mat) -> c_int;                    
    pub fn cv_mat_data(mat: *const Cv2Mat) -> *mut c_uchar;
    pub fn cv_imencode(ext: *const c_char, img: *const Cv2Mat, params: *const c_int) -> *mut ImgBuffer;
    pub fn cv_free_img_buffer(buf: *mut ImgBuffer) -> c_void;

}
#[link(name = "webcam", kind = "static")]
extern "C" {
    pub fn helloTest() -> c_void;
}
