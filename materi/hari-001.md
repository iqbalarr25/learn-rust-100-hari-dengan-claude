# Hari 1: Setup, Hello Rust, cargo, komentar

⏱ ±1 jam: baca rangkuman 15 menit, latihan 45 menit.

## Rangkuman

### 1. Bikin proyek
```
cargo new hello_rust
cd hello_rust
```
Perintah ini bikin 3 file:
- `Cargo.toml`: info proyek dan daftar library. Mirip `go.mod`.
- `.gitignore`
- `src/main.rs`: file yang pertama dijalankan. Semua kode harus ada di folder `src`.

Di Rust, "proyek" disebut **package**. Istilah ini bakal sering muncul nanti.

> Buat challenge ini **nggak perlu `cargo new`**, proyeknya udah ada (`rust-100-hari`). Kalau mau nyoba, jalanin di `C:\Users\iqbal\RustProjects\`, jangan di dalam folder challenge.

### 2. Isi `main.rs`
```rust
fn main() {
    println!("Hello, world!");
}
```
- `fn` dipakai buat bikin fungsi, sama kayak `func` di Go.
- `main` adalah titik awal program, sama kayak di Go.
- `println!` itu **macro**, bukan fungsi. Cirinya ada tanda `!`. Untuk sekarang anggap aja fungsi biasa. Fungsinya mencetak teks lalu pindah baris, mirip `fmt.Println`.
- **Setiap statement wajib diakhiri `;`.** Go nambahin titik koma otomatis, Rust nggak. Lupa `;` = error.
- Indentasi standar Rust adalah **4 spasi**. Go pakai tab.

Nyisipin nilai ke teks pakai `{}`, mirip `%v` di `fmt.Printf`:
```rust
println!("Halo, {}!", "Iqbal");      // Halo, Iqbal!
println!("{} + {} = {}", 1, 2, 3);   // 1 + 2 = 3
```

### 3. Jalanin & build

| Perintah | Fungsinya | Padanan Go |
|---|---|---|
| `cargo run` | compile + langsung jalanin. Dipakai sehari-hari waktu develop | `go run .` |
| `cargo build` | compile aja, hasilnya file exe di `target/debug/` | `go build` |
| `cargo build --release` | compile versi **optimized** (lebih cepat & kecil) buat production, hasilnya di `target/release/` | `go build` (Go nggak punya mode debug/release) |
| `cargo check` | cuma cek error, nggak bikin exe. **Paling cepat** | mirip `go vet` |

Kalau di-build tanpa `--release`, hasilnya mode **debug**: compile-nya cepat dan ada info buat debugging, tapi program jalannya lebih lambat. Makanya buat production selalu pakai `--release`.

Ada juga `rustc` (compiler aslinya, contoh `rustc hello.rs`), tapi jarang dipakai langsung karena `cargo` udah manggil `rustc` di belakang layar. Cukup tahu aja.

### 4. Komentar
```rust
// komentar satu baris, mirip Go

/* komentar
   banyak baris, juga mirip Go */
```
Komentar diabaikan compiler, jadi nggak ngaruh ke hasil program.

**Tips:** kalau lo pakai VS Code / RustRover dengan plugin `rust-analyzer`, ada tombol **▶ Run** di atas `fn main()`. Klik itu sama aja kayak `cargo run`.

### 5. Khusus proyek challenge ini
Proyek ini punya banyak program, satu file per hari di `src/bin/`. Jadi jalaninnya harus sebut nama harinya:
```
cargo run --bin hari001
```

## Soal
Kerjain di [src/bin/hari001.rs](../src/bin/hari001.rs).

1. **Perkenalan.** Cetak 3 baris: nama lo, bahasa yang udah lo kuasai, dan alasan lo belajar Rust. Pakai `{}` minimal sekali.
2. **Sengaja error.** Hapus tanda `;` di ujung salah satu `println!` (selain yang terakhir), lalu jalanin `cargo run --bin hari001`. Baca pesan errornya dan tulis dalam **satu komentar** di file: menurut lo error itu bilang apa? Habis itu benerin.
3. **debug vs release.** Jalanin `cargo build` lalu `cargo build --release`. Cari `hari001.exe` di `target/debug/` dan `target/release/`, terus jalanin dua-duanya langsung tanpa cargo. Tulis di komentar: bedanya dua folder itu apa, dan kapan pakai yang mana?

## Selesai kalau
- `cargo run --bin hari001` jalan tanpa error dan mencetak 3 baris.
- Ada komentar jawaban soal 2 dan 3 di file.

Kalau udah, bilang **"selesai hari 1"**.

## Bacaan tambahan (opsional)
Rangkuman di atas diambil dari bab ini. Nggak wajib dibaca:
- [A.1 Hello Rust](https://dasarpemrogramanrust.novalagung.com/basic/hello-rust)
- [A.2 Build dan Run Program Rust](https://dasarpemrogramanrust.novalagung.com/basic/build-dan-run-program-rust)
- [A.3 Komentar](https://dasarpemrogramanrust.novalagung.com/basic/komentar)
