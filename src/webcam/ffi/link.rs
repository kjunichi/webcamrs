#[cfg(not(windows))]
#[link(name = "opencv_highgui")]
#[link(name = "opencv_videoio")]
#[link(name = "opencv_imgcodecs")]
#[link(name = "opencv_core")]
extern {}

#[cfg(windows)]
#[link(name = "opencv_world342")]
extern {}