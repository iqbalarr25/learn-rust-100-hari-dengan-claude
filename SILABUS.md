# Silabus 100 Hari Rust

±1 jam per hari. ★ = hari review + mini proyek. Centang = sudah direview dan lulus.

## Fase 1: Dasar bahasa (hari 1–20)
- [x] 1. Setup, Hello Rust, cargo, komentar
- [x] 2. Variabel, `mut`, shadowing
- [ ] 3. Tipe data scalar, casting `as`
- [ ] 4. `&str`, konstanta, operator
- [ ] 5. `if` sebagai ekspresi
- [ ] 6. `while`, `loop`, `break` bawa nilai, label
- [ ] 7. `for in`, range
- [ ] 8. Array & slice
- [ ] 9. Tuple & Vector
- [ ] 10. ★ Mini proyek: rekap nilai siswa
- [ ] 11. Function, ekspresi vs statement
- [ ] 12. Module system: path, crate, module
- [ ] 13. `String` vs `&str`
- [ ] 14. Struct
- [ ] 15. Method & associated function (`impl`)
- [ ] 16. Enum
- [ ] 17. `match` & pattern matching
- [ ] 18. `Option`
- [ ] 19. `Result` & operator `?`
- [ ] 20. ★ Mini proyek: todo list CLI (in-memory)

## Fase 2: Ownership & tipe lanjutan (hari 21–40)
- [ ] 21. Ownership: move
- [ ] 22. `Clone` & `Copy`
- [ ] 23. Borrowing (`&`)
- [ ] 24. Mutable reference (`&mut`) & aturan pinjam
- [ ] 25. Ownership di parameter fungsi
- [ ] 26. Lifetime dasar (kenalan aja)
- [ ] 27. HashMap
- [ ] 28. Iterator: `iter`, `map`, `filter`, `collect`
- [ ] 29. Closure
- [ ] 30. ★ Mini proyek: penghitung kata dari teks
- [ ] 31. Trait: definisi & implementasi
- [ ] 32. Trait bawaan: `Debug`, `Display`, `Default`, `derive`
- [ ] 33. Generic
- [ ] 34. Trait bound & `impl Trait`
- [ ] 35. Custom error pakai enum
- [ ] 36. `From`/`Into` & konversi error dengan `?`
- [ ] 37. Testing: `#[test]`, `cargo test`
- [ ] 38. Pecah proyek jadi beberapa file
- [ ] 39. Crate eksternal & serde_json
- [ ] 40. ★ Mini proyek: baca JSON → olah → tulis file

## Fase 3: Smart pointer & concurrency (hari 41–55)
- [ ] 41. `Box`
- [ ] 42. `Rc` & `RefCell` (kenalan)
- [ ] 43. Thread: `std::thread::spawn`
- [ ] 44. `Arc` & `Mutex`
- [ ] 45. Channel (`mpsc`)
- [ ] 46. `Send` & `Sync` (konsep)
- [ ] 47. `anyhow` & `thiserror`
- [ ] 48. CLI dengan `clap`
- [ ] 49. Logging dengan `tracing`
- [ ] 50. ★ Mini proyek: hitung baris banyak file secara paralel
- [ ] 51. `clippy`, `rustfmt`, cara baca error compiler
- [ ] 52. `if let`, `let else`, `while let`
- [ ] 53. Iterator lanjutan: `enumerate`, `zip`, `fold`, `chain`
- [ ] 54. Bikin Iterator sendiri
- [ ] 55. ★ Review fase 1–3: refactor kode lama

## Fase 4: Async (hari 56–68)
- [ ] 56. Kenapa async: goroutine vs future
- [ ] 57. tokio: `#[tokio::main]`, `async fn`, `.await`
- [ ] 58. `tokio::spawn` & `JoinHandle`
- [ ] 59. Channel tokio: `mpsc`, `oneshot`
- [ ] 60. ★ Mini proyek: TCP echo server
- [ ] 61. `select!` & timeout
- [ ] 62. Shared state di async
- [ ] 63. HTTP client: `reqwest` + serde
- [ ] 64. File I/O async
- [ ] 65. Graceful shutdown
- [ ] 66. Error async umum: "future is not Send"
- [ ] 67. Review async
- [ ] 68. ★ Mini proyek: cek status banyak URL paralel + timeout

## Fase 5: Backend dengan axum (hari 69–88)
- [ ] 69. axum hello world & routing
- [ ] 70. Extractor: `Path`, `Query`, `Json`
- [ ] 71. Response & status code
- [ ] 72. State: `Arc<AppState>`
- [ ] 73. ★ Mini proyek: API todo in-memory (CRUD)
- [ ] 74. Error → `IntoResponse`
- [ ] 75. Validasi input
- [ ] 76. Middleware: `tower-http` (trace, cors, timeout)
- [ ] 77. Config dari env
- [ ] 78. sqlx + Postgres: koneksi & pool
- [ ] 79. Query, `query_as`, migrasi
- [ ] 80. ★ Mini proyek: API todo pakai Postgres
- [ ] 81. Transaksi
- [ ] 82. Pagination & filter
- [ ] 83. Struktur proyek: handler / service / repo
- [ ] 84. Hash password (argon2)
- [ ] 85. JWT & middleware auth
- [ ] 86. Integration test API
- [ ] 87. Docker multi-stage build
- [ ] 88. ★ Review fase 5

## Fase 6: Production & capstone (hari 89–100)
- [ ] 89. Kafka consumer pakai `rdkafka`
- [ ] 90. Batch insert ke Postgres
- [ ] 91. Redis cache
- [ ] 92. Background job
- [ ] 93. Health check & metrics
- [ ] 94. Benchmark: versi Go vs Rust
- [ ] 95. Capstone: desain (rewrite `kafka-to-postgres`)
- [ ] 96. Capstone: consumer
- [ ] 97. Capstone: sink ke Postgres
- [ ] 98. Capstone: test
- [ ] 99. Capstone: deploy
- [ ] 100. ★ Retrospektif: Go vs Rust dari pengalaman sendiri
