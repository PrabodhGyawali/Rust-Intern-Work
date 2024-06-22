# Numerical Quiz Game in Rust

### Guess the song:
1. Run the applications with:
    - `cd numerical-quize`
    - `cargo run -- sounds\data.json`

### Learning Objectives:
Understanding the first principles of audio how High-level language allows you to go from Digital (mp3 files) to analog (streaming through the speakers of the computer).

Used `rodio` library in rust to interface with our computer audio.
1. As my environment is WSL 2, I had to manually configure WSL to be able to recognize I have hardware to output sound. After some googling I needed to install the package: `libasound2-dev`
2. Another useful package was `alsa-utils` for the command `aplay -l`
    - This displays sound recorder and player for  ALSA *(advanced linux sound arch)* soundcard drivers