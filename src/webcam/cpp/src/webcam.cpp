#include <iostream>
#include <string>
#include <vector>

#include "opencv2/opencv.hpp"

using namespace std;
using namespace cv;

extern "C"
{
    typedef struct CvVideoCapture_
    {
        void *raw_ptr;
    } CvVideoCapture;

    typedef struct Cv2Mat_
    {
        void *raw_ptr;
    } Cv2Mat;

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

    Cv2Mat *cv_create_mat() {
        Cv2Mat *mat;
        mat = (Cv2Mat*)malloc(sizeof(Cv2Mat));
        mat->raw_ptr = new Mat();
        return mat;
    }
    
    void cv_read(CvVideoCapture *cap, Cv2Mat *frame) {
        Mat *mat = (Mat*)(frame->raw_ptr);
        ((VideoCapture*)(cap->raw_ptr))->read(*mat);
    }
    
    int cv_mat_cols(Cv2Mat *mat) {
       Mat *frame = ((Mat*)(mat->raw_ptr));
       return frame->cols;
    }

    unsigned char *cv_mat_data(Cv2Mat *mat) {
        Mat *frame = ((Mat*)(mat->raw_ptr));
        return frame->data;
    }

    ImgBuffer *cv_imencode(const char *ext, Cv2Mat *img, int *params) {
       vector<uchar> *buf;
       buf = new vector<uchar>;
       imencode(ext, *((Mat*)(img->raw_ptr)), *buf, vector<int>());
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

    void cv_imshow(const char*winname, Cv2Mat *mat) {
        Mat frame = *((Mat*)(mat->raw_ptr));
        if(!frame.empty()) {
            imshow(winname, frame);
        }
    }

    void cv_release_video_capture(CvVideoCapture *cap) {
        ((VideoCapture*)(cap->raw_ptr))->release();
        delete cap;
    }

    int cv_imwrite(const char *filename, Cv2Mat *mat) {
        Mat frame = *((Mat*)(mat->raw_ptr));
        imwrite(filename,frame);
        return 0;
    }

    CvVideoCapture *cv_video_capture(int camnum) {
        VideoCapture *cap;
        cap = new VideoCapture();
        cap->open(camnum);
        CvVideoCapture *ccap;
        ccap = (CvVideoCapture*)malloc(sizeof(CvVideoCapture));
        ccap->raw_ptr = cap;
        return ccap;
        
        //return ccap;
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
