# 📱➡️💻 Camera Streaming Application

A camera streaming solution that allows you to stream your mobile device's camera to your local computer, creating a virtual camera interface.

## Application Demo

https://github.com/user-attachments/assets/46bdfb98-7640-466c-a39c-788da56d2bfa

## 📋 Prerequisites

Before building and running this application, ensure you have the following dependencies installed:

### 🔧 System Dependencies

#### v4l2loopback (Linux)
This project requires `v4l2loopback` to create virtual camera devices on your system.

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install v4l2loopback-dkms v4l2loopback-utils
```

#### 🦀 Rust Toolchain
Install Rust from the official website or using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### ⚙️ C++ Compilation Toolchain
The build script compiles FFmpeg, which requires a C++ compiler and build tools.

**Ubuntu/Debian:**
```bash
sudo apt install build-essential cmake pkg-config
```

#### ☕ Java Development Kit (JDK)
Required for Android application compilation.

**Ubuntu/Debian:**
```bash
sudo apt install openjdk-11-jdk
```

## 🔨 Building the Application

### Server (Computer)
```bash
git clone https://github.com/Rafael-Conde/local_camera_streaming
cd local_camera_streaming/video-receiver
./build.sh
```

### Android Application
In order to build the application one must add the `key.properties` file in the `src-tauri/gen/android/key.properties` with the fields:

```
password=key-store-and-key-alias-password
keyAlias=key-alias-name
storeFile=/path/to/keystore.jks
```

A keystore and key can be generated with a `keytool`.

```bash
cd local_camera_streaming/camera_streamer
./build.sh
```

install the application by connecting your phone to the computer and using `adb install src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`(adb is android debug bridge tool)

## 🚀 Setup and Usage

### 1. 🌐 Find Your Local IP Address

**Linux:**
```bash
ip addr show | grep inet
# or
ifconfig | grep inet
```

Look for your local network IP address (usually starts with 192.168.x.x or 10.x.x.x).

### 2. 🖥️ Start the Video Receiver Server

On your computer, run the video receiver server:

```bash
./build.sh # this build script already runs the application
```

The server will start listening for incoming video streams on the default port(8080).

### 3. 📱 Configure the Mobile Application

1. Install the Android APK on your mobile device
2. Open the application
4. Enter your computer's local IP address (found in step 1)
5. Ensure the port matches the server configuration (default: 8080)

### 4. ▶️ Start Streaming

1. On your mobile device, tap "Start Streaming"
2. Grant camera permissions when prompted
3. The video stream should now appear as a virtual camera on your computer

### 5. 📹 Using the Virtual Camera

The virtual camera will be available in video conferencing applications, streaming software, and other applications that support camera input. Look for a device named similar to "Rust Virtual Camera" or "v4l2loopback" in your application's camera settings.

## 🔧 Troubleshooting

### Virtual Camera Not Detected
- Ensure v4l2loopback is properly installed and loaded:
  ```bash
  sudo modprobe v4l2loopback
  ```

### Connection Issues
- Verify both devices are on the same network
- Check firewall settings on your computer
- Ensure the IP address and port are correctly configured

### Build Errors
- Make sure all dependencies are installed
- Try cleaning and rebuilding:
  ```bash
  cargo clean
  ./build.sh
  ```

## 🌐 Network Requirements

- Both devices must be connected to the same local network (WiFi/Ethernet)
- Firewall should allow incoming connections on the configured port
- Stable network connection recommended for smooth video streaming

## 💻 Supported Platforms

- **Server:** Linux

## 📄 License

GNU GPLv3(to be added the file, due to ffmpeg build used)
