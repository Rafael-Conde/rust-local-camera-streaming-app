use anyhow::Result;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::codec::Parameters;
use ffmpeg_next::{
    Rational, Rescale,
    codec::Id as CodecId,
    format, frame,
    software::scaling::{context::Context as Scaler, flag::Flags as ScaleFlags},
    util::{color::Range as ColorRange, format::pixel::Pixel},
};

pub struct VideoVirtualCamera {
    output_context: Option<ffmpeg::format::context::Output>,
    encoder: Option<ffmpeg::codec::encoder::video::Encoder>,
    out_stream_index: Option<usize>,
    in_time_base: Option<Rational>,
    enc_time_base: Option<Rational>,
    detected_avg_fps: Option<Rational>,
    width: u32,
    height: u32,
    params: Option<Parameters>,

    // New fields for virtual camera specifics:
    device_path: String,    // e.g., "/dev/video2"
    use_mjpeg: bool,        // true -> MJPEG, false -> RAWVIDEO (YUYV)
    target_pix_fmt: Pixel,  // yuv420p for MJPEG, yuyv422 for rawvideo
    scaler: Option<Scaler>, // frame format/size converter
}

impl VideoVirtualCamera {
    pub fn new(width: u32, height: u32, device_path: impl Into<String>) -> Result<Self> {
        Ok(Self {
            output_context: None,
            encoder: None,
            out_stream_index: None,
            in_time_base: None,
            enc_time_base: None,
            detected_avg_fps: None,
            width,
            height,
            params: None,

            device_path: device_path.into(),
            use_mjpeg: true,                // default to MJPEG for compatibility
            target_pix_fmt: Pixel::YUV420P, // good default for MJPEG
            scaler: None,
        })
    }

    // Optionally let callers switch to rawvideo (YUYV) if needed:
    pub fn use_rawvideo_yuyv422(&mut self) {
        self.use_mjpeg = false;
        self.target_pix_fmt = Pixel::YUYV422;
    }

    pub fn set_parameters(&mut self, params: Parameters) {
        self.params = Some(params);
    }

    // Call once before frames: propagates time base and fps
    pub fn prepare_from_stream(&mut self, in_stream: &ffmpeg::format::stream::Stream) {
        let in_tb = in_stream.time_base();
        let avg_fr = in_stream.avg_frame_rate(); // may be 0/0 on VFR

        self.in_time_base = Some(in_tb);

        let has_cfr = avg_fr.numerator() > 0 && avg_fr.denominator() > 0;
        if has_cfr {
            self.detected_avg_fps = Some(avg_fr);
            self.enc_time_base = Some(Rational::new(avg_fr.denominator(), avg_fr.numerator()));
        } else {
            // If VFR, v4l2loopback generally behaves better with CFR; choose a sane default (e.g., 30fps)
            self.detected_avg_fps = Some(Rational::new(30, 1));
            self.enc_time_base = Some(Rational::new(1, 30)); // 1/30
        }
    }

    pub fn show_frame(&mut self, frame: &ffmpeg::frame::Video) -> Result<()> {
        if self.output_context.is_none() {
            self.init_virtual_camera(frame)?;
        }
        self.render_frame(frame)?;
        Ok(())
    }

    fn ensure_scaler(&mut self, src_w: u32, src_h: u32, src_fmt: Pixel) -> Result<()> {
        if self.scaler.is_none() {
            // Create scaler to convert to target size/format expected by encoder/device
            let scale = Scaler::get(
                src_fmt,
                src_w,
                src_h,
                self.target_pix_fmt,
                self.width,
                self.height,
                ScaleFlags::BILINEAR,
            )?;
            self.scaler = Some(scale);
        }
        Ok(())
    }

    fn init_virtual_camera(&mut self, first_frame: &ffmpeg::frame::Video) -> Result<()> {
        let enc_tb = self.enc_time_base.ok_or_else(|| {
            anyhow::anyhow!(
                "enc_time_base not set. Call prepare_from_stream() before sending frames."
            )
        })?;

        // Open v4l2 output: explicit muxer name is "video4linux2" (short name "v4l2" also works)
        let mut output = format::output_as(&self.device_path, "video4linux2")?;

        // Choose codec: MJPEG or RAWVIDEO
        let codec_id = if self.use_mjpeg {
            ffmpeg::codec::Id::MJPEG
        } else {
            ffmpeg::codec::Id::RAWVIDEO
        };

        let codec = ffmpeg::encoder::find(codec_id)
            .ok_or_else(|| anyhow::anyhow!("Requested encoder not found: {:?}", codec_id))?;

        // Build encoder context
        let mut enc_ctx = ffmpeg::codec::context::Context::new_with_codec(codec)
            .encoder()
            .video()?;

        // Set negotiated size/pix_fmt (v4l2loopback will accept many, but MJPEG+YUV420P is a safe default)
        enc_ctx.set_width(self.width.max(1));
        enc_ctx.set_height(self.height.max(1));
        enc_ctx.set_format(self.target_pix_fmt);

        // FPS -> frame_rate + time_base
        if let Some(avg_fps) = self.detected_avg_fps {
            enc_ctx.set_frame_rate(Some((
                avg_fps.numerator() as i32,
                avg_fps.denominator() as i32,
            )));
        }
        enc_ctx.set_time_base(enc_tb);

        // Optional: tune mjpeg
        // enc_ctx.set_bit_rate(8_000_000); // tweak as desired

        // For v4l2 output, we still add a stream so avformat can negotiate correctly
        let encoder = enc_ctx.open_as(codec)?;

        let stream_index;
        {
            let mut stream = output.add_stream(codec)?;
            stream.set_parameters(&encoder);
            stream_index = stream.index();
        }

        // v4l2 output has no global header/trailer needs, but writing header is fine
        output.write_header()?;

        // Prime scaler if needed
        self.ensure_scaler(
            first_frame.width(),
            first_frame.height(),
            first_frame.format(),
        )?;

        self.out_stream_index = Some(stream_index);
        self.encoder = Some(encoder);
        self.output_context = Some(output);

        Ok(())
    }

    // fn render_frame(&mut self, frame: &ffmpeg::frame::Video) -> Result<()> {
    //     if let (Some(encoder), Some(output_context)) = (&mut self.encoder, &mut self.output_context)
    //     {
    //         let enc_tb = self.enc_time_base.expect("enc_time_base should be set");
    //         let in_tb = self.in_time_base.expect("in_time_base should be set");
    //
    //         // Convert to target format/size if needed
    //         {
    //             let src_w = frame.width();
    //             let src_h = frame.height();
    //             let src_fmt = frame.format();
    //             if self.scaler.is_none() {
    //                 // Create scaler to convert to target size/format expected by encoder/device
    //                 let scale = Scaler::get(
    //                     src_fmt,
    //                     src_w,
    //                     src_h,
    //                     self.target_pix_fmt,
    //                     self.width,
    //                     self.height,
    //                     ScaleFlags::BILINEAR,
    //                 )?;
    //                 self.scaler = Some(scale);
    //             }
    //             Result::<(), anyhow::Error>::Ok(())
    //         }?;
    //         let scaler = self.scaler.as_mut().unwrap();
    //
    //         let mut converted = frame::Video::empty();
    //         converted.set_format(self.target_pix_fmt);
    //         converted.set_width(self.width);
    //         converted.set_height(self.height);
    //
    //         scaler.run(frame, &mut converted)?;
    //
    //         // PTS handling: rescale to encoder time base
    //         if let Some(src_pts) = frame.pts() {
    //             let dst_pts = src_pts.rescale(in_tb, enc_tb);
    //             converted.set_pts(Some(dst_pts));
    //         }
    //
    //         // Send to encoder
    //         encoder.send_frame(&converted)?;
    //
    //         // Drain packets
    //         let mut packet = ffmpeg::Packet::empty();
    //         while encoder.receive_packet(&mut packet).is_ok() {
    //             let out_idx = self.out_stream_index.unwrap_or(0);
    //             let out_tb = output_context.stream(out_idx).unwrap().time_base();
    //
    //             // For devices, timestamps are not always pivotal, but keep them consistent
    //             packet.rescale_ts(enc_tb, out_tb);
    //             packet.set_stream(out_idx);
    //             packet.write_interleaved(output_context)?;
    //         }
    //     }
    //     Ok(())
    // }

    fn to_full_range_for_mjpeg(pix: Pixel) -> Pixel {
        match pix {
            Pixel::YUV420P => Pixel::YUVJ420P,
            Pixel::YUV422P => Pixel::YUVJ422P,
            Pixel::YUV444P => Pixel::YUVJ444P,
            // Common semi-planar inputs -> go to a JPEG-compatible target
            Pixel::NV12 | Pixel::NV21 => Pixel::YUVJ420P,
            // If already full-range "yuvj", keep it
            Pixel::YUVJ420P | Pixel::YUVJ422P | Pixel::YUVJ444P => pix,
            // Default: keep original (if not YUV or not compatible, swscale will still convert)
            other => other,
        }
    }

    fn is_mjpeg_encoder(enc: &ffmpeg::codec::encoder::video::Encoder) -> bool {
        enc.codec()
            .map(|c| c.id() == CodecId::MJPEG)
            .unwrap_or(false)
    }

    pub fn render_frame(&mut self, frame: &ffmpeg::frame::Video) -> Result<()> {
        if let (Some(encoder), Some(output_context)) = (&mut self.encoder, &mut self.output_context)
        {
            let enc_tb = self.enc_time_base.expect("enc_time_base should be set");
            let in_tb = self.in_time_base.expect("in_time_base should be set");

            // Decide the destination pixel format:
            // - If encoding to MJPEG, use a full-range "yuvj" format so the encoder gets full-range YUV.
            // - Otherwise, keep the configured target format.
            let dest_pix_fmt = if Self::is_mjpeg_encoder(encoder) {
                Self::to_full_range_for_mjpeg(self.target_pix_fmt)
            } else {
                self.target_pix_fmt
            };

            // Build/rebuild scaler if needed
            {
                let src_w = frame.width();
                let src_h = frame.height();
                let src_fmt = frame.format();

                // If your struct stores 'self.scaler' only, we at least ensure it's created.
                // If you want to be strict about re-creation when out fmt changes, optionally
                // store 'self.scaler_out_fmt' and recreate when it differs from dest_pix_fmt.
                if self.scaler.is_none() {
                    let scale = Scaler::get(
                        src_fmt,
                        src_w,
                        src_h,
                        dest_pix_fmt,
                        self.width,
                        self.height,
                        ScaleFlags::BILINEAR,
                    )?;
                    self.scaler = Some(scale);
                }
            }
            let scaler = self.scaler.as_mut().unwrap();

            // Prepare destination frame with JPEG/full-range metadata.
            let mut converted = frame::Video::empty();
            converted.set_format(dest_pix_fmt);
            converted.set_width(self.width);
            converted.set_height(self.height);

            // Hint the source range to the scaler if it’s present on the input frame.
            // This increases the chance swscale will expand limited -> full as part of conversion.
            // (If the source doesn't have it set, swscale will assume limited for typical YUV.)
            converted.set_color_range(frame.color_range()); // swscale will consult both frames’ metadata

            // Do the scaling/pixfmt conversion (this is where range expansion occurs if needed)
            scaler.run(frame, &mut converted)?;

            // Now explicitly mark the output as full range (JPEG) for correctness and encoder compliance.
            converted.set_color_range(ColorRange::JPEG);

            // PTS handling: rescale to encoder time base
            if let Some(src_pts) = frame.pts() {
                let dst_pts = src_pts.rescale(in_tb, enc_tb);
                converted.set_pts(Some(dst_pts));
            }

            // Send to encoder
            encoder.send_frame(&converted)?;

            // Drain packets
            let mut packet = ffmpeg::Packet::empty();
            while encoder.receive_packet(&mut packet).is_ok() {
                let out_idx = self.out_stream_index.unwrap_or(0);
                let out_tb = output_context.stream(out_idx).unwrap().time_base();
                packet.rescale_ts(enc_tb, out_tb);
                packet.set_stream(out_idx);
                packet.write_interleaved(output_context)?;
            }
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<()> {
        if let (Some(encoder), Some(output_context)) = (&mut self.encoder, &mut self.output_context)
        {
            let enc_tb = self.enc_time_base.unwrap_or(Rational(1, 1000));
            let out_idx = self.out_stream_index.unwrap_or(0);
            let out_tb = output_context.stream(out_idx).unwrap().time_base();

            encoder.send_eof()?;

            let mut packet = ffmpeg::Packet::empty();
            while encoder.receive_packet(&mut packet).is_ok() {
                packet.rescale_ts(enc_tb, out_tb);
                packet.set_stream(out_idx);
                packet.write_interleaved(output_context)?;
            }

            // v4l2 doesn’t need a trailer, but calling it is harmless
            output_context.write_trailer()?;
        }
        Ok(())
    }
}

impl Drop for VideoVirtualCamera {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}
