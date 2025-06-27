# Overlog

**Overlog** is a terminal-based tool that overlays telemetry data (GPS, speed, altitude, g-force, etc.) onto video files or renders transparent overlays for live use in OBS Studio. Built with Rust and FFmpeg, it's optimized for low-latency and high-quality data visualization.

![Overlog Demo](demo.gif)

---

## 🚀 Features

- 📍 Parse telemetry logs (CSV, JSON, GPX, etc.)
- 🛰️ Visualize GPS paths, speed graphs, g-force rings
- 🎥 Burn telemetry overlay into MP4 using FFmpeg
- 🧊 Export transparent overlay video (.webm or .mov)
- 🎛️ Integrate with OBS Studio as media/browser source
- 💡 CLI-first design with optional GUI frontend

---

## 🧰 Tech Stack

- **Rust** – performance and safety
- **FFmpeg** – video decoding/encoding
- **Serde** – structured telemetry parsing
- **image**, **plotters**, **rusttype** – for drawing
- **GeoRust**, **geojson**, **proj** – for location data
- **Optionally**: Web frontend using WASM + Canvas for live rendering

---

## 📦 Installation

```bash
git clone https://github.com/makalin/overlog.git
cd overlog
cargo build --release
````

Ensure `ffmpeg` is installed and in your `PATH`.

---

## 🧪 Usage

### 1. Parse telemetry

```bash
overlog parse data.gpx > out.json
```

### 2. Render transparent overlay

```bash
overlog render --input out.json --output overlay.webm
```

### 3. Burn overlay into MP4

```bash
ffmpeg -i input.mp4 -i overlay.webm -filter_complex "[0:v][1:v] overlay=0:0" -c:a copy output.mp4
```

---

## 🖥️ Use with OBS Studio

1. Run `overlog` to generate a `.webm` or `.mov` overlay
2. Add it as a Media Source in OBS
3. Loop or sync it with your main video

---

## 📁 Supported Formats

* Telemetry input: `.csv`, `.json`, `.gpx`, `.tcx`, GoPro `.bin` (soon)
* Video: `.mp4`, `.mov`, `.webm`, `.avi`

---

## 📌 Roadmap

* [ ] Live overlay sync with GPS timestamp
* [ ] Audio-reactive overlay effects
* [ ] Native OBS plugin (Rust/WASM)
* [ ] HTML5 export for browser playback

---

## 🧠 Inspiration

This project is inspired by tools like:

* [GoPro Telemetry Extractor](https://goprotelemetryextractor.com/)
* [RaceRender](https://racerender.com/)
* [Dashware](https://www.dashware.net/)

But Overlog runs in your terminal, is open-source, and lightweight ⚡

---

## 📝 License

MIT © 2025 [@makalin](https://github.com/makalin)
