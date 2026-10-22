# Hari 2: Variabel, `mut`, shadowing

⏱ ±1 jam: baca rangkuman 20 menit, latihan 40 menit.

## Rangkuman

### 1. Bikin variabel pakai `let`
```rust
let nama = "Iqbal";        // tipe ditebak compiler (type inference)
let umur: i32 = 25;        // tipe ditulis sendiri: nama: tipe
```

| Go | Rust |
|---|---|
| `nama := "Iqbal"` | `let nama = "Iqbal";` |
| `var umur int32 = 25` | `let umur: i32 = 25;` |

- Nama variabel pakai **snake_case**: `user_id`, `login_count`. Bukan `userId` kayak di Go.
- Rust itu *statically typed* kayak Go, jadi tipe semua variabel udah ketahuan waktu compile. Tipe-tipe angka (`i32`, `u8`, dll.) dibahas besok.

### 2. Default-nya NGGAK bisa diubah (immutable)
Ini beda paling gede dari Go. Di Go semua variabel bisa diubah. Di Rust kebalikannya:
```rust
let status = "pending";
status = "paid";           // ❌ ERROR: cannot assign twice to immutable variable

let mut status = "pending"; // pakai `mut` = mutable (boleh diubah)
status = "paid";            // ✅
```
**Kenapa begitu?** Biar lo sadar mana data yang memang berubah. Kalau ada `mut`, pembaca kode langsung tahu "nah, ini bakal diubah di bawah". Kalau nggak ada `mut`, nilainya dijamin tetap sampai akhir.

Kebiasaan yang bagus: **jangan pakai `mut` kecuali memang perlu.**

### 3. Deklarasi dulu, isi belakangan
```rust
let role: &str;            // belum ada isinya
role = "admin";            // diisi sekali → boleh, walaupun nggak `mut`
// role = "user";          // ❌ isi kedua kali → error
```
Pakai variabel yang belum diisi juga error. Di Go, variabel kosong otomatis dapat *zero value* (`0`, `""`, `nil`). **Rust nggak punya zero value otomatis**, jadi lo wajib ngisi dulu sebelum dipakai.

### 4. Banyak variabel sekaligus
```rust
let (kode, pesan) = (200, "OK");                 // mirip Go: kode, pesan := 200, "OK"
let (a, mut b) = (1, 2);                         // cuma b yang bisa diubah
let (lebar, tinggi): (i32, i32) = (1920, 1080);  // tipe ditulis per posisi
```
Bentuk `(200, "OK")` ini namanya **tuple**, yaitu beberapa nilai yang dibungkus jadi satu. Dibahas di Hari 9.

### 5. Shadowing: bikin ulang variabel dengan nama yang sama
```rust
let username = "  IqbalArr  ";
let username = username.trim();   // variabel BARU, nama sama, isi "IqbalArr"
```
`let` yang kedua **bukan ngubah** variabel lama. Dia bikin variabel baru yang "menutupi" (*shadow*) variabel lama.

Bedanya sama `mut`: **shadowing boleh ganti tipe**, `mut` nggak boleh.
```rust
let total = "  42  ";          // tipe: teks
let total = total.trim();      // masih teks: "42"
let total = total.len();       // sekarang ANGKA (panjang teks = 2) ✅

let mut x = "abc";
x = x.len();                   // ❌ ERROR: tipe harus tetap teks
```
`.trim()` = buang spasi di awal & akhir (kayak `strings.TrimSpace` di Go). `.len()` = panjang teks (kayak `len()` di Go).

Kalau di Go, `x := ...` dua kali di scope yang sama bakal error *"no new variables"*. Di Rust ini justru dipakai buat "membersihkan" data step by step, misalnya input dari request.

### 6. Cetak variabel di `println!`
```rust
let nama = "Iqbal";
let kota = "Jakarta";

println!("{} dari {}", nama, kota);     // urut sesuai argumen
println!("{1} dari {0}", nama, kota);   // pakai nomor posisi → "Jakarta dari Iqbal"
println!("{nama} dari {kota}");         // nama variabel langsung di dalam {} ✨
// println!(nama);                      // ❌ argumen pertama WAJIB teks dalam tanda kutip
```
Cara ketiga (`{nama}`) paling enak dibaca, jadi mulai sekarang pakai itu aja kalau bisa.

### 7. Variabel nggak dipakai → cuma warning
```rust
let token = "abc";          // ⚠️ warning: unused variable `token`
let _token = "abc";         // awalan _ = "sengaja nggak dipakai", warning hilang
let _ = hitung_sesuatu();   // buang hasil fungsi
```
Di Go, variabel nggak kepakai = **compile error**. Di Rust cuma **warning**, jadi program tetap jalan. Tapi warning tetap harus dibereskan.

## Soal
Kerjain di [src/bin/hari002.rs](../src/bin/hari002.rs).

1. **Data user.** Bikin variabel *immutable* `nama`, `email`, `umur` (tipe `i32` ditulis eksplisit), dan `is_active` (isi `true`). Bikin juga `let (status_code, status_text) = ...` dari tuple `(200, "OK")`. Cetak semuanya pakai gaya `{nama}`, contoh output:
   ```
   [200 OK] Iqbal <iqbal@mail.com>, umur 25, aktif: true
   ```
   Terus bikin `login_count` yang mulai dari 0, naikin 3 kali pakai `+= 1`, lalu cetak `login_count: 3`.
2. **Sengaja error.** Tambahin satu baris yang ngubah `email` jadi email lain, tanpa `mut`. Jalanin, baca errornya, lalu tulis di komentar pakai bahasa lo sendiri: error-nya bilang apa, dan saran `help:`-nya apa? Habis itu **hapus** baris tadi. Jangan nambahin `mut`, karena email user di sini memang nggak perlu berubah.
3. **Bersihin input pakai shadowing.** Anggap ini input dari form register:
   ```rust
   let username = "   IqbalArr   ";
   ```
   - Cetak pakai kurung siku biar spasinya kelihatan: `[   IqbalArr   ]`
   - Shadow jadi versi yang udah di-`trim()`, terus cetak lagi: `[IqbalArr]`
   - Shadow **sekali lagi** jadi panjangnya (`.len()`), terus cetak: `panjang username: 8`
   - Di komentar, jawab: kenapa langkah terakhir bisa pakai shadowing tapi **nggak bisa** pakai `mut`?

## Selesai kalau
- `cargo run --bin hari002` jalan **tanpa error dan tanpa warning**.
- Output soal 1 dan 3 sesuai contoh.
- Ada komentar jawaban soal 2 dan 3.

Kalau udah, bilang **"selesai hari 2"**.

## Bacaan tambahan (opsional)
- [A.4 Variabel](https://dasarpemrogramanrust.novalagung.com/basic/variabel)
