function install_v4l2loopback() {
    sudo apt install v4l2loopback-dkms v4l2loopback-utils
}

sudo modprobe v4l2loopback video_nr=2 exclusive_caps=1 card_label="Rust Virtual Cam" max_buffers=2
