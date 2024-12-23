use webcamrs::webcam::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};

extern crate webcamrs;

fn main() {
    let name = "WebCam test";
    /*
    let capture = webcamrs::webcam::create_camera_capture(0);
    webcamrs::webcam::named_window(name);
    loop {
        let frame = webcamrs::webcam::query_frame(&capture);
        webcamrs::webcam::show_image(name, &frame);
        let c = webcamrs::webcam::wait_key(2);
        if c == 0x1b {
            break;
        }
        if c == 0x20 {
            webcamrs::webcam::save_image("snap.jpg", &frame);
            let params = vec![];
            let mat = webcamrs::webcam::encode_image(".ppm", &frame, params);
            println!("size = {}, rows ={} ", mat.cols, mat.rows);
            let v = mat.buf;
            for i in 0..12 {
                println!("vec[{}] = {} ", i, v[i] as char);
            }
            break;
        }
    }
    webcamrs::webcam::release_capture(&capture);
    webcamrs::webcam::destroy_all_windows();
    */
    //webcamrs::webcam::helloTest();
    //webcamrs::webcam::destroy_all_windows();
    webcamrs::webcam::named_window(name);
    let cap = webcamrs::webcam::video_capture_with_api_preference(0,webcamrs::webcam::CAP_DSHOW );
    let frame = webcamrs::webcam::create_mat();
    cap.set(CAP_PROP_FRAME_WIDTH,1280.0);
    cap.set(CAP_PROP_FRAME_HEIGHT,720.0);
    loop {
        webcamrs::webcam::read(&cap, &frame);
        //let frame = webcamrs::webcam::read(&cap);
        webcamrs::webcam::imshow(name, &frame);
        let c = webcamrs::webcam::wait_key(20);
        if c == 0x1b {
            break;
        }
        if c == 0x20 {
            webcamrs::webcam::imwrite("snap.jpg", &frame);
            let params = vec![];
            let buf = webcamrs::webcam::imencode(".ppm", &frame, params);
            for i in 0..12 {
                println!("vec[{}] = {} ", i, buf[i] as char);
            }
            break;
        }
    }
    webcamrs::webcam::release(&cap);
    webcamrs::webcam::destroy_all_windows();
    println!()
}
