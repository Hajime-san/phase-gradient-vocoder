# phase-gradient-vocoder

## Enviroment
- 5.10.102.1-microsoft-standard-WSL2 x86_64 GNU/Linux
- rustup 1.25.1 (bb60b1e89 2022-07-12)  
- rustc 1.65.0 (897e37553 2022-11-02)

## Support format
- Waveform Audio File
    - read
        - compression code
            - Linear PCM
            - MS-ADPCM
            - IBM CSVD
    - write
        - channel
            - 1 channel

## Commands
- Compile source code and run binary  
    - time stretch  
    `cargo run --release -- --mode time-stretch --ratio 0.8`
    - pitch shift  
    `cargo run --release -- --mode pitch-shift --ratio 1.3`
