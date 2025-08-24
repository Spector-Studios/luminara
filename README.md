# Luminara
[![Build and Deploy](https://github.com/Spector-Studios/luminara/actions/workflows/build.yml/badge.svg)](https://github.com/Spector-Studios/luminara/actions/workflows/build.yml)

A Strategy RPG game written in Rust with [Macroquad](https://github.com/not-fl3/macroquad) developed entirely on a phone with [Termux](https://github.com/termux/termux-app)

Currently, only Android and Web on Mobile are supported platforms.

## Screenshots
<img width="1080" height="2408" alt="Screenshot_20250824_091923_Chrome" src="https://github.com/user-attachments/assets/1aad9439-f4d2-4bd5-a9c5-082cfb7a0eb2" />

## Building
### Web
```shell
./wasm-build.py
```
This will produce `dist` directory in project root which can be hosted by an HTTP server. Pass `-r` flag to start a dev server.

### Android
Install [`cargo-ndk`](https://github.com/bbqsrc/cargo-ndk) with:
```
cargo binstall cargo-ndk
or
cargo install cargo-ndk
```
Assuming you have Android SDK and NDK installed at usual locations, run:
```
./gradlew assembleDebug
```

**On Termux:**
TODO
