// Hari 1: lihat materi/hari-001.md
// Jalanin: cargo run --bin hari001

fn main() {
    // Soal 1: cetak 3 baris perkenalan di sini
    println!("Halo namaku {}", "Iqbal");
    println!("aku pernah menguasai bahasa java dan dart, sekarang aku mainly di php,golang, dan sekarang belajar rust");
    println!("aku ingin menguasai rust karna saya melihat bahasa ini sangat kuat sebagai bahasa pemograman, dan setelah mendengar keunggulan rust dari claude saya semakin semangat belajar")
    // Soal 2: jawaban (komentar)   
    //error: expected `;`, found `println` (itu karena baris ga ditutup pake ; jadi dia anggep baris selanjutnya masih 1 baris dan terjadilah error. yang unik nya aku temuin sebenernya ga wajib pake ; kalo dia ada diakhir code, cuman biar aman harusnya selalu dipake)
    // Soal 3: jawaban (komentar)
    // kalo cargo build itu buat staging karna masih unoptimized dan ada debug info, yang ku tau konsepnya mirip di flutter pas build apk size nya gede dan ada debug info ketika ada error (contohnya overflow keliatan)
    // kalo cargo build --release itu buat prod size kecil dan lebih cepat karna gada debug info
}
