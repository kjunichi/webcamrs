#[cfg(not(windows))]
#[link(name = "opencv_highgui")]
#[link(name = "opencv_videoio")]
#[link(name = "opencv_imgcodecs")]
extern {}

#[cfg(windows)]
#[link(name = "opencv_world320")]
extern {}