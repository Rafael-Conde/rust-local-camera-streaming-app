fn main() {
    let ffmpeg_dir = "/home/rafael/old_ffmpeg_build";

    // Add library search path
    println!("cargo:rustc-link-search=native={}/lib", ffmpeg_dir);

    // Force link all FFmpeg libraries
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=avdevice");
    println!("cargo:rustc-link-lib=static=avfilter");
    println!("cargo:rustc-link-lib=static=swscale");
    println!("cargo:rustc-link-lib=static=swresample");

    // Add system dependencies that FFmpeg needs
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
}
