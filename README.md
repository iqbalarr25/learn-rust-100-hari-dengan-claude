# Learn Rust 100 Hari dengan Claude 🦀

Challenge 100 hari belajar Rust dari nol, dipandu [Claude](https://claude.com/claude-code). Progres harian ada di [SILABUS.md](SILABUS.md).

Target: dari nol sampai bisa bikin backend Rust (axum + sqlx + Postgres + Kafka), ±1 jam per hari.

## Cara kerja
1. Bilang ke Claude **"gas hari N"** → Claude bikin `materi/hari-NNN.md` (materi + soal) dan `src/bin/hariNNN.rs` (tempat ngerjain).
2. Baca materinya, kerjain soalnya di `src/bin/hariNNN.rs`, jalanin:
   ```
   cargo run --bin hariNNN
   ```
3. Bilang **"selesai hari N"** → Claude review kodenya. Kalau lulus, dicentang di [SILABUS.md](SILABUS.md) dan baru lanjut ke hari berikutnya.
4. Commit & push sendiri:
   ```
   git add .
   git commit -m "hari N: <topik>"
   git push
   ```

## Aturan
- "Hari" = sesi belajar, bukan tanggal. Skip sehari karena sibuk? Nggak apa-apa, lanjut aja besoknya.
- Coba sendiri minimal 15 menit sebelum nanya.
- Baca error compiler sampai habis, terutama baris `help:`.
- Nyangkut sama borrow checker di awal? `.clone()` dulu, rapihin belakangan.

## Sumber
- [Dasar Pemrograman Rust (novalagung)](https://dasarpemrogramanrust.novalagung.com/) untuk fase 1–2 (bahasa Indonesia).
- Materi dari Claude untuk fase 3–6 (async, backend, production).
