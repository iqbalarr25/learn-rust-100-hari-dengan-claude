// Hari 2: lihat materi/hari-002.md
// Jalanin: cargo run --bin hari002

fn main() {
    // Soal 1: data user + login_count
    let nama = "Iqbal Arrafi";
    let email = "iqbalarrafi39@gmail.com";
    let umur: i32 = 24;
    let is_active = true;
    let (status_code, status_text) = (200, "OK");

    println!("[{status_code} {status_text}] {nama} <{email}>, umur {umur}, aktif: {is_active}");

    let mut login_count = 0;
    login_count += 1;
    login_count += 1;
    login_count += 1;
    println!("login_count = {login_count}");
    // Soal 2: jawaban (komentar)
    // cannot assign twice to immutable variable `email` cannot assign twice to immutable variable
    // intinya variable yang immutable gabisa di assign lebih dari satu kali, kalau emang mau di ubah data nya pake mut (mutable)
    // sarannya nyuruh ubah ke mut (mutable)
    // Soal 3: bersihin input username pakai shadowing
    let username = "   IqbalArr   ";
    println!("[{username}]");
    let username = username.trim();
    println!("[{username}]");
    let username = username.len();
    println!("panjang username: {username}");
    //karna itu bukan ngubah variabel yang udah ada, tapi niban
}
