import { useEffect, useState, useCallback } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface RustLog {
  log_message: string;
}

class MediaRecorderStreamer {
  video: HTMLVideoElement;
  mediaStream: MediaStream | null;
  mediaRecorder: MediaRecorder | null;
  websocket: WebSocket | null;
  networkAddr: string;

    constructor() {
        this.video = document.getElementById('video') as HTMLVideoElement;
        this.mediaStream = null;
        this.mediaRecorder = null;
        this.websocket = null;
        this.networkAddr = "";
    }

    async startCamera() {
        try {
            this.mediaStream = await navigator.mediaDevices.getUserMedia({
                video: { width: 640, height: 480, frameRate: { ideal: 30 } },
                audio: false
            });
            
            this.video.srcObject = this.mediaStream;
            return true;
        } catch (error) {
            console.error('Error accessing camera:', error);
            return false;
        }
    }

    async startStreaming(serverUrl = 'ws://localhost:8080') {
        try {
            // Start camera
            await this.startCamera();
            console.log("Started camera");
            
            // Connect WebSocket
            this.websocket = new WebSocket(serverUrl);
            await new Promise((resolve, reject) => {
                this.websocket!.onopen = resolve;
                this.websocket!.onerror = reject;
            });

            console.log("Started ws");
            
            // Setup MediaRecorder
            this.mediaRecorder = new MediaRecorder(this.mediaStream!, {
                mimeType: 'video/webm;codecs=h264',
                videoBitsPerSecond: 500000 // 1 Mbps
            });

            console.log("Started mr");
            
            this.mediaRecorder.ondataavailable = (event) => {
                if (event.data.size > 0 && this.websocket!.readyState === WebSocket.OPEN) {
                    this.websocket!.send(event.data);
                }
            };

            console.log("set callback");
            
            // Start recording with small time slices
            this.mediaRecorder.start(100); // Send data every 100ms
            console.log("set 100");
            
            return true;
        } catch (error: any) {
            console.error('Error starting stream:', error.message);
            return false;
        }
    }

    stopStreaming() {
        if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
            this.mediaRecorder.stop();
        }
        
        if (this.websocket) {
            this.websocket.close();
        }
        
        if (this.mediaStream) {
            this.mediaStream.getTracks().forEach(track => track.stop());
        }
    }
}

function App() {
  // const [responseMsg, setResponseMsg] = useState("");
  const [addr, setAddr] = useState("");
  const [logString, setLogString] = useState("");

  const streamer = new MediaRecorderStreamer();

  const start_streaming = useCallback(async () => {
    setLogString(logString + "\n" + "[FRONTEND] calling start stream");
    setLogString(logString + "\n" + await invoke('start_server', { device: "This shouldn't be used", tcpAddr: addr }));
    setLogString(logString + "\n" + "[FRONTEND] Exiting function call");
  }, [addr]);

  listen<RustLog>("rust-log", useCallback((event) => {
    setLogString(logString + "\n" + event.payload.log_message)
  }, [logString, setLogString, addr]));


  listen("server-running", useCallback(async (_event) => {
    setLogString(logString + "\n" + "[FRONTEND] Starting streaming");
    await streamer.startStreaming();
    setLogString(logString + "\n" + "[FRONTEND] Started");
  }, [logString, setLogString, addr]));

  useEffect(() => {
    streamer.networkAddr = addr;
  }, [addr])

  return (
    <main className="container">
      <h1>Streaming...</h1>
      <video id="video"
        autoPlay
        muted></video>

      <form
        className="row"
        onSubmit={ async (e) => {
          e.preventDefault();
          await start_streaming();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setAddr(e.currentTarget.value)}
          placeholder="Enter a addr..."
        />
        <button type="submit">Start Stream</button>
      </form>
      <p>Current Addr: {addr}</p>
      <p>Rust Logs:</p>
      <div style={{whiteSpace: 'pre-wrap'}}>{logString}</div>
    </main>
  );
}

export default App;
