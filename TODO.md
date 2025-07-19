# 🗂️ Roadmap Brostman — Versi Belajar + Developer Tools

## ✅ 0. Setup (DONE)
- [x] Init project Rust
- [x] Setup RustRover + toolchain
- [x] Tambah egui + eframe
- [x] Bisa kirim request GET
- [x] Rename ke Brostman dan publish ke GitHub

## 🚧 1. Core HTTP Client Features (Minimal Postman clone)
Fase ini buat bikin Brostman bisa dipake dasar-dasarnya

- [ ] Support POST, PUT, DELETE, PATCH
- [ ] Input headers key-value (multiline table)
- [ ] Input body (JSON/text)
- [ ] Tampilkan response status, headers, dan body
- [ ] Syntax highlight response JSON
- [ ] Auto-format (pretty print) JSON

## 🧠 2. UX & State Management
Biar app makin enak dipakai

- [ ] Auto-focus URL input saat startup
- [ ] Simpan history request (di file JSON lokal)
- [ ] Tombol "Copy Response", "Clear"
- [ ] Validasi: warn kalau URL kosong
- [ ] Loading spinner saat request jalan

## 📁 3. Struktur Proyek + Kode Lebih Rapi
Biar makin nyaman buat belajar dan kontribusi

- [ ] Pisah komponen UI: `header_panel.rs`, `body_panel.rs`, `response_panel.rs`
- [ ] Buat `RequestState` struct buat nyimpan semua input
- [ ] Buat adapter HTTP client (`reqwest`, `ureq`, dll) di file terpisah
- [ ] Tambah `mod.rs` di tiap folder

## ✨ 4. Fitur Tambahan (Opsional tapi Keren)
- [ ] Support cURL import
- [ ] Tabbed request (seperti Postman tab)
- [ ] Save + Load workspace
- [ ] Dark mode toggle (kalau belum default)

## 🔧 5. Dev Tools + Publikasi
- [ ] Tambah logging (log request/response)
- [ ] Build release binary (via `cargo build --release`)
- [ ] Upload ke GitHub Releases
- [ ] Tambah dokumentasi fitur di README
