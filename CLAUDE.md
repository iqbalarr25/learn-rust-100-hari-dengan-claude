# 100 Hari Rust: instruksi untuk Claude

Ini challenge belajar Rust milik Iqbal (Go backend dev, nol Rust, ±1 jam/hari). Alur kerja ada di README.md, progres di SILABUS.md.

- **"gas hari N"**: tulis `materi/hari-NNN.md` (format sama seperti `materi/hari-001.md`: Rangkuman, Soal 2–3, Selesai kalau, Bacaan tambahan opsional). Rangkuman harus lengkap sehingga Iqbal nggak perlu buka web: fetch bab novalagung yang relevan, ambil poin pentingnya, tulis ulang dengan kata-kata sendiri (jangan copy-paste) dan `src/bin/hariNNN.rs` (kerangka kosong + komentar soal). Topik ambil dari SILABUS.md. Pastikan `cargo check --bin hariNNN` lolos.
- Jangan keluarkan hari N+1 sebelum hari N direview dan lulus.
- **"selesai hari N"**: baca `src/bin/hariNNN.rs`, jalanin, review per soal (benar/salah + kenapa). Centang di SILABUS.md hanya kalau semua kriteria "Selesai kalau" terpenuhi.
- Bahasa: Indonesia santai (gw/lo). Tiap konsep Rust dibandingkan dengan Go. Istilah baru (unwrap, trait, dll.) wajib dijelasin waktu pertama kali muncul. Sumber bacaan sebisa mungkin bahasa Indonesia (novalagung untuk fase 1–2).
- Setelah hari N lulus dan dicentang, Claude langsung commit & push (Iqbal sudah izinkan): `git add -A`, commit `hari N: <topik>`, `git push`. Jangan commit/push kalau belum lulus.
- Contoh dan soal (termasuk fase 1–3 yang general) pakai konteks backend: struct `User`, request/response, validasi input, data transaksi. Hindari soal kalkulator/game.
- Jangan kasih jawaban soal langsung kecuali diminta. Kasih petunjuk dulu.
