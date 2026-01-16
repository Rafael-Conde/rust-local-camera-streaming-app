FFMPEG_BUILD_DIR=$HOME/old_ffmpeg_build
FFMPEG_SOURCE_DIR=$HOME/old_ffmpeg_sources
OUTPUT_BINDIR=$HOME/bin_old_ffmpeg

function clean_old_ffmpeg_build() {
  sudo rm -r $FFMPEG_SOURCE_DIR $FFMPEG_BUILD_DIR $OUTPUT_BINDIR
}

function build_ffmpeg_old() {
  # installing dependencies
  sudo apt-get update -qq && sudo apt-get -y install \
    autoconf \
    llvm \
    automake \
    build-essential \
    cmake \
    git-core \
    libass-dev \
    libfreetype6-dev \
    libgnutls28-dev \
    libmp3lame-dev \
    libsdl2-dev \
    libtool \
    libva-dev \
    libvdpau-dev \
    libvorbis-dev \
    libxcb1-dev \
    libxcb-shm0-dev \
    libxcb-xfixes0-dev \
    meson \
    ninja-build \
    pkg-config \
    texinfo \
    wget \
    yasm \
    zlib1g-dev \
    libunistring-dev

  mkdir -p $FFMPEG_SOURCE_DIR $OUTPUT_BINDIR

  # building nasm from source
  cd $FFMPEG_SOURCE_DIR && \
  wget https://www.nasm.us/pub/nasm/releasebuilds/2.16.01/nasm-2.16.01.tar.bz2 && \
  tar xjvf nasm-2.16.01.tar.bz2 && \
  cd nasm-2.16.01 && \
  ./autogen.sh && \
  PATH="$HOME/bin:$PATH" ./configure --prefix="$FFMPEG_BUILD_DIR" --bindir="$HOME/bin" && \
  make -j$(nproc) && \
  make install

  # installing libx264
  cd $FFMPEG_SOURCE_DIR && \
  git -C x264 pull || git clone --depth 1 https://code.videolan.org/videolan/x264.git && \
  cd x264 && \
                                                                                                                                                                       # This disables the shared libraries and 
                                                                                                                                                                       # So that we make sure of statically linking 
                                                                                                                                                                       # ffmpeg in the final build
  PATH="$HOME/bin:$PATH" PKG_CONFIG_PATH="$FFMPEG_BUILD_DIR/lib/pkgconfig" ./configure --prefix="$FFMPEG_BUILD_DIR" --bindir="$HOME/bin" --enable-static --enable-pic --disable-shared && \
    PATH="$HOME/bin:$PATH" make -j$(nproc) && \
  make install

  # installing libx265
  sudo apt install libnuma-dev && \
  cd $FFMPEG_SOURCE_DIR && \
  wget -O x265.tar.bz2 https://bitbucket.org/multicoreware/x265_git/get/master.tar.bz2 && \
  tar xjvf x265.tar.bz2 && \
  cd multicoreware*/build/linux && \
  PATH="$HOME/bin:$PATH" cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="$FFMPEG_BUILD_DIR" -DENABLE_SHARED=off ../../source && \
  PATH="$HOME/bin:$PATH" make -j$(nproc) && \
  make install

  # Installing libvpx
  cd $FFMPEG_SOURCE_DIR && \
  git -C libvpx pull 2> /dev/null || git clone --depth 1 https://chromium.googlesource.com/webm/libvpx.git && \
  cd libvpx && \
  PATH="$HOME/bin:$PATH" ./configure --prefix="$FFMPEG_BUILD_DIR" --disable-examples --disable-unit-tests --enable-vp9-highbitdepth --as=yasm --disable-shared --enable-static && \
  PATH="$HOME/bin:$PATH" make -j$(nproc) && \
  make install

  # Installing libfdk-aac
  cd $FFMPEG_SOURCE_DIR && \
  git -C fdk-aac pull 2> /dev/null || git clone --depth 1 https://github.com/mstorsjo/fdk-aac && \
  cd fdk-aac && \
  autoreconf -fiv && \
  ./configure --prefix="$FFMPEG_BUILD_DIR" --disable-shared && \
  make -j$(nproc) && \
  make install

  # Installing libopus
  cd $FFMPEG_SOURCE_DIR && \
  git -C opus pull 2> /dev/null || git clone --depth 1 https://github.com/xiph/opus.git && \
  cd opus && \
  ./autogen.sh && \
  ./configure --prefix="$FFMPEG_BUILD_DIR" --disable-shared && \
  make -j$(nproc) && \
  make install

  # Installing libaom
  cd $FFMPEG_SOURCE_DIR && \
  git -C aom pull 2> /dev/null || git clone --depth 1 https://aomedia.googlesource.com/aom && \
  mkdir -p aom_build && \
  cd aom_build && \
  PATH="$HOME/bin:$PATH" cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="$FFMPEG_BUILD_DIR" -DENABLE_TESTS=OFF -DENABLE_NASM=on ../aom && \
  PATH="$HOME/bin:$PATH" make -j$(nproc) && \
  make install

  # Installing libsvtav1
  cd $FFMPEG_SOURCE_DIR && \
  git -C SVT-AV1 pull 2> /dev/null || git clone https://gitlab.com/AOMediaCodec/SVT-AV1.git && \
  # mkdir -p SVT-AV1/build && \
  cd SVT-AV1/Build/linux && \
  # PATH="$HOME/bin:$PATH" cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="$FFMPEG_BUILD_DIR" -DCMAKE_BUILD_TYPE=Release -DBUILD_DEC=OFF -DBUILD_SHARED_LIBS=OFF .. && \
  ./build.sh release jobs=$(nproc) --prefix="$FFMPEG_BUILD_DIR" --bindir="$FFMPEG_BUILD_DIR/bin" --enable-lto --enable-pgo --static --native --install 
  # cd ../../
  # cp -r ./Bin/Release/* $FFMPEG_BUILD_DIR
  # PATH="$HOME/bin:$PATH" make && \
  # make install

  # Installing libdav1d
  cd $FFMPEG_SOURCE_DIR && \
  # pip3 install meson
  git -C dav1d pull 2> /dev/null || git clone --depth 1 https://code.videolan.org/videolan/dav1d.git && \
  mkdir -p dav1d/build && \
  cd dav1d/build && \
  PATH="$PATH:$HOME/bin" meson setup -Denable_tools=false -Denable_tests=false --default-library=static .. --prefix "$FFMPEG_BUILD_DIR" --libdir="$FFMPEG_BUILD_DIR/lib" && \
  PATH="$PATH:$HOME/bin" ninja && \
  PATH="$PATH:$HOME/bin" ninja install

  # Installing libvmaf
  cd $FFMPEG_SOURCE_DIR && \
  wget https://github.com/Netflix/vmaf/archive/v3.0.0.tar.gz && \
  tar xvf v3.0.0.tar.gz && \
  mkdir -p vmaf-3.0.0/libvmaf/build &&\
  cd vmaf-3.0.0/libvmaf/build && \
  PATH="$PATH:$HOME/bin" meson setup -Denable_tests=false -Denable_docs=false --buildtype=release --default-library=static .. --prefix "$FFMPEG_BUILD_DIR" --bindir="$FFMPEG_BUILD_DIR/bin" --libdir="$FFMPEG_BUILD_DIR/lib" && \
  PATH="$PATH:$HOME/bin" ninja && \
  PATH="$PATH:$HOME/bin" ninja install
  cp -r $FFMPEG_BUILD_DIR/bin/* ~/bin

  # # Installing cuda codec
  # cd $FFMPEG_SOURCE_DIR && \
  # git clone https://git.videolan.org/git/ffmpeg/nv-codec-headers.git
  # cd nv-codec-headers
  # sudo make install


  # building finally ffmpeg
  cd $FFMPEG_SOURCE_DIR && \
  git clone https://git.ffmpeg.org/ffmpeg.git && \
  cd ffmpeg && \
  git checkout n8.0 && \
  PATH="$HOME/bin:$PATH" PKG_CONFIG_PATH="$FFMPEG_BUILD_DIR/lib/pkgconfig" ./configure \
    --prefix="$FFMPEG_BUILD_DIR" \
    --pkg-config-flags="--static" \
    --extra-cflags="-I$FFMPEG_BUILD_DIR/include -I/usr/local/cuda/include" \
    --extra-ldflags="-L$FFMPEG_BUILD_DIR/lib -L/usr/local/cuda/lib64" \
    --extra-libs="-lpthread -lm" \
    --ld="g++" \
    --bindir="$HOME/bin" \
    --enable-gpl \
    --enable-gnutls \
    --enable-libaom \
    --enable-libass \
    --enable-libfdk-aac \
    --enable-libfreetype \
    --enable-libmp3lame \
    --enable-libopus \
    --enable-libsvtav1 \
    --enable-libdav1d \
    --enable-libvorbis \
    --enable-libvpx \
    --enable-libx264 \
    --enable-libx265 \
    --enable-nonfree \
    --enable-static \
    --enable-lto && \
    PATH="$HOME/bin:$PATH" make -j$(nproc) && \
  make install && \
  hash -r
}


  # PATH="$HOME/bin:$PATH" PKG_CONFIG_PATH="$FFMPEG_BUILD_DIR/lib/pkgconfig" ./configure \
  #   --prefix="$FFMPEG_BUILD_DIR" \
  #   --pkg-config-flags="--static" \
  #   --extra-cflags="-I$FFMPEG_BUILD_DIR/include -I/usr/local/cuda/include" \
  #   --extra-ldflags="-L$FFMPEG_BUILD_DIR/lib -L/usr/local/cuda/lib64" \
  #   --extra-libs="-lpthread -lm" \
  #   --ld="g++" \
  #   --bindir="$HOME/bin" \
  #   --enable-gpl \
  #   --enable-gnutls \
  #   --enable-libaom \
  #   --enable-libass \
  #   --enable-libfdk-aac \
  #   --enable-libfreetype \
  #   --enable-libmp3lame \
  #   --enable-libopus \
  #   --enable-libsvtav1 \
  #   --enable-libdav1d \
  #   --enable-libvorbis \
  #   --enable-libvpx \
  #   --enable-libx264 \
  #   --enable-libx265 \
  #   --enable-nonfree \
  #   --enable-static \
  #   --enable-lto && \

# build_ffmpeg_old
