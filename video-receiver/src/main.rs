use anyhow::Result;
use clap::Parser;
use ffmpeg_next::{self as ffmpeg, Dictionary};

use video_receiver::display::VideoVirtualCamera;

#[derive(Parser)]
#[command(name = "video-receiver")]
#[command(about = "A CLI app that receives video over TCP and displays it")]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Host to bind to(currently unused, sorry :'[)
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Window width
    #[arg(short, long, default_value = "480")]
    width: u32,

    /// Window height
    #[arg(short = 'H', long, default_value = "640")]
    height: u32,

    /// Virtual Camera device path
    #[arg(short, long, default_value = "/dev/video2")]
    device_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎥 Video Receiver CLI");
    println!("📡 Listening on {}:{}", args.host, args.port);

    // Initialize FFmpeg
    ffmpeg::init()?;

    println!("✅ Server started, waiting for connections...");
    let receiver = VideoReceiver::new(args.width, args.height);
    loop {
        receiver.handle_connection(args.device_path.as_str(), args.port)?;
    }
}

struct VideoReceiver {
    width: u32,
    height: u32,
}

impl VideoReceiver {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn handle_connection(&self, device_path: &str, port: u16) -> Result<()> {
        println!("🎬 Starting video playback...");

        // Create the display; it will later be prepared using the input stream timing.
        let mut display = VideoVirtualCamera::new(self.width, self.height, device_path)?;
        display.use_rawvideo_yuyv422();
        println!("VideoDisplay initialized");

        // Open a listening TCP input. You can also put listen=1 in the URL query, but this works:
        let mut dict = Dictionary::new();
        dict.set("listen", "1");

        let url = format!("tcp://{}:{}", "0.0.0.0", port);
        let mut ictx = ffmpeg::format::input_with_dictionary(&url, dict)?;
        println!("Input context opened: {}", url);

        // Find the first video stream, create a decoder from its parameters,
        // and prepare the display from that stream (parameters + timebase + framerate).
        let mut video_stream_index: Option<usize> = None;
        let mut decoder_opt = None;

        for stream in ictx.streams() {
            if stream.parameters().medium() == ffmpeg::media::Type::Video {
                video_stream_index = Some(stream.index());

                // Prepare the display using the discovered stream timing/parameters.
                // This should set encoder/output time_base and frame rate inside VideoDisplay.
                display.prepare_from_stream(&stream);

                // Create the decoder from the stream parameters
                let ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
                let decoder = ctx.decoder().video()?;
                decoder_opt = Some(decoder);
                println!(
                    "Using video stream index {} | time_base={}/{} | avg_frame_rate={}/{}",
                    stream.index(),
                    stream.time_base().numerator(),
                    stream.time_base().denominator(),
                    stream.avg_frame_rate().numerator(),
                    stream.avg_frame_rate().denominator(),
                );
                break;
            }
        }

        let Some(video_stream_index) = video_stream_index else {
            return Err(anyhow::anyhow!(
                "Didn't find a video stream in the incoming connection"
            ));
        };

        let Some(mut decoder) = decoder_opt else {
            return Err(anyhow::anyhow!(
                "Couldn't create a decoder for the incoming video stream"
            ));
        };

        println!("Decoder ready, starting demux/decode loop…");

        // Reusable frame
        let mut frame = ffmpeg::frame::Video::empty();

        // Demux packets and send to decoder
        for (stream, packet) in ictx.packets() {
            if stream.index() != video_stream_index {
                continue;
            }

            // Send packet to decoder; ignore EAGAIN, report only real errors
            if let Err(err) = decoder.send_packet(&packet) {
                // EAGAIN means the decoder needs draining; we'll handle it by receive loop below
                if err.to_string().contains("Resource temporarily unavailable") {
                    // benign
                } else {
                    eprintln!("Error sending packet to decoder: {err}");
                }
            }

            // Receive and process all frames available after this packet
            loop {
                match decoder.receive_frame(&mut frame) {
                    Ok(()) => {
                        // Let the display handle timing/encoding/muxing.
                        if let Err(err) = display.show_frame(&frame) {
                            eprintln!("Error displaying/encoding frame: {err}");
                        }
                    }
                    Err(err) => {
                        // Break only on EAGAIN (no more frames right now) or EOF
                        let msg = err.to_string();
                        if msg.contains("Resource temporarily unavailable")
                            || msg.contains("End of file")
                        {
                            break;
                        } else {
                            eprintln!("Error receiving frame: {err}");
                            break;
                        }
                    }
                }
            }
        }

        // Flush the decoder at end of stream and drain remaining frames
        if let Err(err) = decoder.send_eof() {
            eprintln!("Error sending EOF to decoder: {err}");
        }

        loop {
            match decoder.receive_frame(&mut frame) {
                Ok(()) => {
                    if let Err(err) = display.show_frame(&frame) {
                        eprintln!("Error displaying/encoding frame (drain): {err}");
                    }
                }
                Err(err) => {
                    // Stop draining on EAGAIN or EOF
                    let msg = err.to_string();
                    if msg.contains("Resource temporarily unavailable")
                        || msg.contains("End of file")
                    {
                        break;
                    } else {
                        eprintln!("Error draining decoder: {err}");
                        break;
                    }
                }
            }
        }

        // Explicitly finish (also happens in Drop, but this forces trailer write now)
        if let Err(err) = display.finish() {
            eprintln!("Error finalizing output: {err}");
        }

        println!("✅ Finished receiving and writing video.");
        Ok(())
    }
}
