# Interessen Pünktchen Sammler

In unserer tollen Kita Struktur darf man jeden Monat eine Mail schreiben um vielleicht (!) einen Kitaplatz zu bekommen.
Also gehört es automatisiert.
Ich weiß das es wahrscheinlich einfacher geht, aber ich freue mich über echte Projekte in Rust umzusetzen.

Das Programm erwartet eine config.yaml als Argument mit folgendem Inhalt:

```yaml
---
mail_server: mail.example.com
smtp_port: 587
username: testuser
password: testpassword
subject: Test Email
destination: dest@examle.com
message: Test Message
```

## Build

```bash
git clone https://github.com/urtokk/interessen_puenktchen_sammler.git
cd interessen_puenktchen_sammler
cargo build --release
```

## Run

```bash
./target/release/interessen_puenktchen_sammler config.yaml
```
