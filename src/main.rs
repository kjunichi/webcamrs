#[allow(dead_code)]
pub mod cv {
    extern crate libc;
    //use self::libc::{ c_char, c_double, c_int, c_schar, c_void }
    use self::libc::{ c_char, c_int, c_void }
    ;
    use std::ffi::CString;

    pub struct Capture;
    pub struct IplImage;

    #[link(name = "opencv_highgui")]
    extern"C" {
        fn cvDestroyAllWindows() -> c_void;
        fn cvNamedWindow(title: * const c_char) -> c_int;
        fn cvShowImage(name: * const c_char, image: * const IplImage) -> c_void;
        fn cvWaitKey(delay: c_int) -> c_int;
        fn cvCreateCameraCapture(index: c_int) -> * mut Capture;
        fn cvQueryFrame(capture: * mut Capture) -> * mut IplImage;
        fn cvSaveImage(filename: * const c_char, image: * const IplImage, params: *const c_int) -> c_int;
        fn cvReleaseCapture(capture: & * mut Capture) -> c_void;
    }

    pub fn destroy_all_windows() -> () {
        unsafe {
            cvDestroyAllWindows();
        }
    }

    pub fn named_window(title: &str) -> () {
        /*
        title.with_c_str(| title | unsafe {
            cvNamedWindow(title);
        }
        );*/
        unsafe {
            cvNamedWindow(CString::new(title).unwrap().as_ptr());
        }
    }

    pub fn create_camera_capture(index: i32) -> * mut Capture {
        unsafe {
            cvCreateCameraCapture(index)
        }
    }

    pub fn query_frame(capture: * mut Capture) -> * mut IplImage {
        unsafe {
            cvQueryFrame(capture)
        }
    }

    pub fn show_image(name: &str, image: * const IplImage) -> () {
        /*name.with_c_str(| name | unsafe {
            cvShowImage(name, image);
        }
        )*/
        unsafe {
            cvShowImage(CString::new(name).unwrap().as_ptr(), image);
        }
    }

    pub fn wait_key(delay: i32) -> i32 {
        unsafe {
            cvWaitKey(delay)
        }
    }

    pub fn save_image(name: &str, image: * const IplImage) -> i32 {
      /*name.with_c_str(| name | unsafe {
          cvSaveImage(name, image,0 as * const i32)
      }
      )*/
      unsafe {
          cvSaveImage(CString::new(name).unwrap().as_ptr(), image,0 as * const i32)
      }
    }

    pub fn release_capture(capture: * mut Capture) -> () {
      unsafe {
        cvReleaseCapture(&capture);
      }
    }

}

fn main() {
    let name = "WebCam test";
    let capture = cv::create_camera_capture(0);
    cv::named_window(name);
    loop {
        let frame = cv::query_frame(capture);
        cv::show_image(name, frame as * const cv::IplImage);
        let c = cv::wait_key(2);
        if c == 0x1b {
            break;
        }
        if c == 0x20 {
          cv::save_image("snap.jpg",frame as * const cv::IplImage);
          break;
        }
    }
    cv::release_capture(capture);
    cv::destroy_all_windows();
}
