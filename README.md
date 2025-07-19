# 🚀 KirimAja

**KirimAja** adalah aplikasi desktop native yang dibuat dengan Rust, terinspirasi dari Postman, Apidog, dan Hoppscotch.  
Tujuan utamanya adalah untuk mengirim dan mengelola HTTP request dengan antarmuka grafis sederhana namun powerful.

> 📌 **Disclaimer**:  
> Project ini dibuat sebagai _learning journey_ gue untuk belajar bahasa Rust.  
> Gue baru mulai belajar, dan KirimAja ini adalah studi kasus pertama gue:  
> **membangun aplikasi desktop mirip Postman tapi versi native, ringan, dan open source**.

---

## 🎯 Fitur Utama (Roadmap)

✅ Kirim HTTP request (`GET`, `POST`, dll)  
✅ Tampilan GUI dengan [egui](https://github.com/emilk/egui)  
✅ Tampilkan response dari server  
⬜ Input custom header  
⬜ Body editor (JSON/raw)  
⬜ Tab request  
⬜ Simpan history & collection ke file  
⬜ Export/import collection  
⬜ Theme mode (dark/light)

---

## 🧱 Stack Teknologi

- 🦀 [Rust](https://www.rust-lang.org/)
- 🖼️ [eframe / egui](https://github.com/emilk/egui): GUI library modern
- 🌐 [reqwest](https://crates.io/crates/reqwest): HTTP client
- 📦 [serde & serde_json](https://serde.rs/): Serialisasi & parsing JSON

---

## 🛠️ Cara Menjalankan

```bash
git clone https://github.com/username/KirimAja.git
cd KirimAja
cargo run
```
Butuh: Rust & Cargo (minimal versi 1.70), dan libssl-dev kalau pakai Linux.

## 📸 Screenshot (optional)

Belum tersedia — coming soon!

## 🙏 Kontribusi & Saran

Karena ini masih early-stage project sambil belajar, semua feedback & saran sangat diterima.
Kalau mau bantuin, feel free buka issue atau pull request ya!

## 📜 Lisensi

KirimAja dirilis di bawah lisensi MIT – silakan digunakan, dimodifikasi, dan dikembangkan secara bebas.

Lihat file [LICENSE](./LICENSE) untuk detail lengkapnya.
