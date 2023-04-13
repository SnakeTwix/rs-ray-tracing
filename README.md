# Russian

Для запуска программы необходимо иметь Cargo и минимальную версию Rust 1.70.0-nightly. Вероятно, что работает и на стабильном релизе.

`cargo run --release` сгенерирует картинку размером 1920х1080.

Если не хочется ждать 30 минут ради одной картинки, то советую изменить `MAX_DEPTH` и `SAMPLES_PER_PIXEL` на меньшее значение. Они лежат в `main.rs`

# English

To run this program you have to have Cargo installed. And some version of rust. Definitely works for 1.70.0-nightly. So if something doesn't work, you do you. 

`cargo run --release` and it should generate an image with a resolution of 1920x1080.

If you don't want to wait for 30 minutes, consider changing `MAX_DEPTH` and `SAMPLES_PER_PIXEL` to a smaller value in `main.rs`
