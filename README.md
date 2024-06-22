# Numerical Quiz Game in Rust

### Guess the song:
1. Run the applications with:
    - `cd numerical-quize`
    - `cargo run -- sounds\data.json`

### Learning Objectives:
Understanding the first principles of audio how High-level language allows you to go from Digital (mp3 files) to analog (streaming through the speakers of the computer).

Used `rodio` library in rust to interface with our computer audio system. However, it didn't work in the first try and I learnt quite a bit on OS and Audio interface in a computer explained below.
```
ALSA lib confmisc.c:855:(parse_card) cannot find card '0'
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1334:(snd_func_refer) error evaluating name
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5701:(snd_config_expand) Evaluate error: No such file or directory
ALSA lib pcm.c:2664:(snd_pcm_open_noupdate) Unknown PCM default
aplay: main:831: audio open error: No such file or directory
```


1. As my environment is WSL 2, I had to manually configure WSL to be able to recognize I have hardware to output sound. After some googling I needed to install the package: `libasound2-dev`
2. Another useful package was `alsa-utils` for the command `aplay -l`
    - This displays sound recorder and player for  [ALSA](https://en.wikipedia.org/wiki/Advanced_Linux_Sound_Architecture) soundcard drivers
3. Deep Dive [Github Thread](https://github.com/microsoft/WSL/issues/7327)
    - Python has a "SoundCard" library using *PulseAudio* Server and WSLg uses PulseAudio Server
4. What is **PulseAudio**? -> it is middleware
    - Applications || MIDDLEWARE || Hardware Devices
    - uses either ALSA or Open sound system (OSS)
        - *zero configuartion networking* with **Avahi**
    - `/etc/pulse` directory on Linux has got PulseAudio configuration files `client.conf`

5. Key terms to look into in `client.conf`
    - `default-sink` - output device (sink) for audio playback
    - `default-source` - input device (source) 
    - `default-server` - address of PulseAudio server for client to connect to (WSL requires IP and port if running on different machine)
        - Configure this to your "tcp:YOUR_WINDOWS_SUBNET_IP:4173" use `ip route` in WSL to find the IP address of your network interface
    - `default-dbus-server` - Desktop bus
    - `grep nameserver /etc/resolv.conf` gives Windows host IP, `ping` it to make sure its correct

6. `ps -aux | grep pulseaudio` clearly proves pulseaudio is a background process ran in WSL

7. Finally run `aplay audio.mp3` 

