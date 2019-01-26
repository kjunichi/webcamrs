#include <iostream>
#include <string>
#include <vector>

#include "opencv2/opencv.hpp"

using namespace std;
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

    typedef struct ImgBuffer_ {
        void *ptr;
        int size;
        void *raw;
    } ImgBuffer;

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
    
    void cv_read(cvVideoCapture cap, cv2Mat frame) {
        int retryCnt=3;
        Mat *mat = (Mat*)(frame.raw_ptr);
        while(retryCnt > 0) {
            (*((VideoCapture*)(cap.raw_ptr))).read(*mat);
            if(!mat->empty()) {
                break;
            }
            retryCnt--;
        }
    }
    
   /*
    cv2Mat cv_read(cvVideoCapture cap) {
        //Mat mat = *((Mat*)(frame.raw_ptr));
        Mat *tmp = new Mat();
        *((VideoCapture*)(cap.raw_ptr)) >> *tmp;
        cv2Mat frame;
        frame.raw_ptr = tmp;
        return frame;
    }
    */
    int cv_mat_cols(cv2Mat mat) {
       Mat *frame = ((Mat*)(mat.raw_ptr));
       return frame->cols;
    }

    unsigned char *cv_mat_data(cv2Mat mat) {
        Mat *frame = ((Mat*)(mat.raw_ptr));
        return frame->data;
    }

    ImgBuffer *cv_imencode(const char *ext, cv2Mat img, int *params) {
       vector<uchar> *buf;
       buf = new vector<uchar>;
       imencode(ext, *((Mat*)(img.raw_ptr)), *buf, vector<int>());
       ImgBuffer *dst = new ImgBuffer();
       dst->ptr = buf->data();
       dst->size = buf->size();
       dst->raw = buf;
       return dst;
    }

    void cv_free_img_buffer(ImgBuffer *buf) {
        delete (vector<uchar>*)(buf->raw);
        delete buf;
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

    int cv_imwrite(const char *filename, cv2Mat mat) {
        Mat frame = *((Mat*)(mat.raw_ptr));
        imwrite(filename,frame);
        return 0;
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
