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

## Links
- [Phase Vocoder Done Right](https://www.eurasip.org/Proceedings/Eusipco/Eusipco2017/papers/1570343436.pdf)
- [REAL-TIME SPECTROGRAM INVERSION USING PHASE GRADIENT HEAP
INTEGRATION](https://ltfat.org/notes/ltfatnote043.pdf)
- [Pitch-shifting algorithm
design and applications in
music](http://kth.diva-portal.org/smash/get/diva2:1381398/FULLTEXT01.pdf)
- [An Open-Source Phase Vocoder with Some
Novel Visualizations](https://music.informatics.indiana.edu/media/students/kyung/kyung_paper.pdf)
- [音響信号処理における位相復元](https://www.jstage.jst.go.jp/article/essfr/15/1/15_25/_pdf/-char/ja)
- [深層学習を用いた声質変換の実装と実験的評価](https://chuo-u.repo.nii.ac.jp/?action=repository_action_common_download&item_id=14844&item_no=1&attribute_id=22&file_no=1)
- [小特集「位相情報を考慮した音声音響信号処理」
にあたって](https://www.jstage.jst.go.jp/article/jasj/75/3/75_125/_pdf)
- [Phase Importance in Speech Processing
Applications](https://www.isca-speech.org/archive_v0/archive_papers/interspeech_2014/i14_1623.pdf)
- [人間の聴覚心理現象と位相の関係](https://www.jstage.jst.go.jp/article/oyama/38/0/38_KJ00004384981/_pdf)

## Licence
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
- This is a fan made content which based on the "The Fan-Made Content Policy"（[https://denonbu.jp/guidelines](https://denonbu.jp/guidelines)）
