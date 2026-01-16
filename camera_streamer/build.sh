# Source this file to get the compilation commands in your shell
CAMERA_STREAMER_PROJECT_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"


export ANDROID_NDK_ROOT="$HOME/Android/Sdk/ndk/27.0.12077973"
export NDK_TOOLCHAIN="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64"
export SYSROOT="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/sysroot"
# eza -la -s=type $NDK_TOOLCHAIN
# export PATH="$NDK_TOOLCHAIN/bin:$PATH"

# cargo ndk build --profile opt
export AR_aarch64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_linux_android="--sysroot=$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot -I$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/aarch64-linux-android"
export CARGO_NDK_SYSROOT_LIBS_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android"
export CARGO_NDK_SYSROOT_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export CARGO_NDK_SYSROOT_TARGET="aarch64-linux-android"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_AR="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
# export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$HOME/.cargo/bin/cargo-ndk"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_RUNNER="$HOME/.cargo/bin/cargo-ndk-runner"
export CC_aarch64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
export CFLAGS_aarch64_linux_android="--target=aarch64-linux-android21"
export CLANG_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CXXFLAGS_aarch64_linux_android="--target=aarch64-linux-android21"
export CXX_aarch64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang++"
export RANLIB_aarch64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib"

# To import with bash/zsh/etc:
#     source <(cargo ndk-env)
export AR_armv7_linux_androideabi="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
export BINDGEN_EXTRA_CLANG_ARGS_armv7_linux_androideabi="--sysroot=$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot -I$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/arm-linux-androideabi"
export CARGO_NDK_SYSROOT_LIBS_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi"
export CARGO_NDK_SYSROOT_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export CARGO_NDK_SYSROOT_TARGET="arm-linux-androideabi"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_AR="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
# export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$HOME/.cargo/bin/cargo-ndk"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_RUNNER="$HOME/.cargo/bin/cargo-ndk-runner"
export CC_armv7_linux_androideabi="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
export CFLAGS_armv7_linux_androideabi="--target=armv7a-linux-androideabi21"
export CLANG_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CXXFLAGS_armv7_linux_androideabi="--target=armv7a-linux-androideabi21"
export CXX_armv7_linux_androideabi="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang++"
export RANLIB_armv7_linux_androideabi="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib"

# To import with bash/zsh/etc:
#     source <(cargo ndk-env)
export AR_x86_64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
export BINDGEN_EXTRA_CLANG_ARGS_x86_64_linux_android="--sysroot=$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot -I$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/x86_64-linux-android"
export CARGO_NDK_SYSROOT_LIBS_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/x86_64-linux-android"
export CARGO_NDK_SYSROOT_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export CARGO_NDK_SYSROOT_TARGET="x86_64-linux-android"
export CARGO_TARGET_X86_64_LINUX_ANDROID_AR="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
# export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$HOME/.cargo/bin/cargo-ndk"
export CARGO_TARGET_X86_64_LINUX_ANDROID_RUNNER="$HOME/.cargo/bin/cargo-ndk-runner"
export CC_x86_64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CFLAGS_x86_64_linux_android="--target=x86_64-linux-android21"
export CLANG_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang"
export CXXFLAGS_x86_64_linux_android="--target=x86_64-linux-android21"
export CXX_x86_64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang++"
export RANLIB_x86_64_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib"

# To import with bash/zsh/etc:
#     source <(cargo ndk-env)
export AR_i686_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
export BINDGEN_EXTRA_CLANG_ARGS_i686_linux_android="--sysroot=$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot -I$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/i686-linux-android"
export CARGO_NDK_SYSROOT_LIBS_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/i686-linux-android"
export CARGO_NDK_SYSROOT_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export CARGO_NDK_SYSROOT_TARGET="i686-linux-android"
export CARGO_TARGET_I686_LINUX_ANDROID_AR="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
# export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="$HOME/.cargo/bin/cargo-ndk"
export CARGO_TARGET_I686_LINUX_ANDROID_RUNNER="$HOME/.cargo/bin/cargo-ndk-runner"
export CC_i686_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang"
export CFLAGS_i686_linux_android="--target=i686-linux-android21"
export CLANG_PATH="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CXXFLAGS_i686_linux_android="--target=i686-linux-android21"
export CXX_i686_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang++"
export RANLIB_i686_linux_android="$HOME/Android/Sdk/ndk/27.0.12077973/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib"

function tauri_build() {
    cd $CAMERA_STREAMER_PROJECT_ROOT
    cargo tauri android build
    cd -
}

function tauri_dev() {
    cd $CAMERA_STREAMER_PROJECT_ROOT
    cargo tauri android dev
    cd -
}
