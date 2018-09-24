#include <iostream>
#include <string>

#include "opencv2/opencv.hpp"

using namespace cv;

extern "C"
{
    typedef struct cvVideoCapture_
    {
        void *raw_ptr;
    } cvVideoCapture;

    typedef struct cv2Mat_
    {
        void *raw_ptr;
    } cv2Mat;

    void cv_destroy_all_windows()
    {
        destroyAllWindows();
    }

    int cv_wait_key(int delay)
    {
        return waitKey(delay);
    }

    void cv_named_window(const char *title)
    {
        namedWindow(title, 1);
    }

    cv2Mat cv_create_mat() {
        cv2Mat *mat;
        mat = (cv2Mat*)malloc(sizeof(cv2Mat));
        mat->raw_ptr = new Mat();
        return *mat;
    }

    /*
    void cv_read(cvVideoCapture cap, cv2Mat frame) {
        //Mat mat = *((Mat*)(frame.raw_ptr));
        Mat tmp;
        *((VideoCapture*)(cap.raw_ptr)) >> tmp;
        frame.raw_ptr = &tmp;
    }
    */
    cv2Mat cv_read(cvVideoCapture cap) {
        //Mat mat = *((Mat*)(frame.raw_ptr));
        Mat *tmp = new Mat();
        *((VideoCapture*)(cap.raw_ptr)) >> *tmp;
        cv2Mat frame;
        frame.raw_ptr = tmp;
        return frame;
    }

    void cv_imshow(const char*winname, cv2Mat mat) {
        Mat frame = *((Mat*)(mat.raw_ptr));
        if(!frame.empty()) {
            imshow(winname, frame);
        }
    }

    void cv_release_video_capture(cvVideoCapture cap) {
        ((VideoCapture*)(cap.raw_ptr))->release();
    }

    cvVideoCapture cv_video_capture(int camnum) {
        VideoCapture *cap;
        cap = new VideoCapture();
        cap->open(camnum);
        cvVideoCapture *ccap;
        ccap = (cvVideoCapture*)malloc(sizeof(cvVideoCapture));
        ccap->raw_ptr = cap;
        return *ccap;
    }

    void helloTest()
    {
        VideoCapture cap(0); // open the default camera
        if (!cap.isOpened()) // check if we succeeded
            return;
        Mat edges;
        namedWindow("edges", 1);
        for (;;)
        {
            Mat frame;
            cap >> frame; // get a new frame from camera
            //cvtColor(frame, edges, COLOR_BGR2GRAY);
            //GaussianBlur(edges, edges, Size(7, 7), 1.5, 1.5);
            //Canny(edges, edges, 0, 30, 3);
            imshow("edges", frame);
            if (waitKey(30) >= 0)
                break;
        }
        std::cout << "hello cpp" << std::endl;
    }
}
