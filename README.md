# StellarSort Core Library

<p align="center">
  <img src="https://user-images.githubusercontent.com/yourusername/logo.png" alt="StellarSort Logo" width="200">
</p>

<p align="center">
  <b>Ein leistungsstarkes Rust-Toolkit zur Analyse und Sortierung von Astrofotografie-Bildern.</b>
</p>

<p align="center">
  <a href="https://crates.io/crates/stellarsort_core">
    <img src="https://img.shields.io/crates/v/stellarsort_core.svg" alt="Crates.io Version">
  </a>
  <a href="https://docs.rs/stellarsort_core">
    <img src="https://docs.rs/stellarsort_core/badge.svg" alt="Documentation">
  </a>
  <a href="https://github.com/yourusername/stellarsort_core/actions">
    <img src="https://github.com/yourusername/stellarsort_core/workflows/CI/badge.svg" alt="Build Status">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/crates/l/stellarsort_core.svg" alt="License">
  </a>
</p>

---

## Inhaltsverzeichnis

- [Überblick](#überblick)
- [Features](#features)
- [Installation](#installation)
- [Verwendung](#verwendung)
  - [Beispiel: Unscharfe Bilder erkennen](#beispiel-unscharfe-bilder-erkennen)
- [API Referenz](#api-referenz)
- [Beitragen](#beitragen)
- [Lizenz](#lizenz)
- [Danksagungen](#danksagungen)

---

## Überblick

**StellarSort Core Library** ist eine leistungsstarke Rust-Bibliothek, die speziell für die Analyse und Sortierung von Astrofotografie-Bildern entwickelt wurde. Sie bietet Werkzeuge zur Erkennung von unscharfen Bildern, Rauschunterdrückung und anderen Bildqualitätsanalysen, um Astrofotografen bei der Verarbeitung ihrer Bilder zu unterstützen.

## Features

- **Unscharfe Bilder erkennen**: Nutzt fortschrittliche Algorithmen, um unscharfe Bilder zuverlässig zu identifizieren.
- **Rauschunterdrückung**: Integrierte Filter zur Reduzierung von Bildrauschen ohne Verlust wichtiger Details.
- **Parallele Verarbeitung**: Beschleunigen Sie die Analyse großer Bildmengen mit Unterstützung für parallele Verarbeitung.
- **Erweiterbar**: Modularer Aufbau ermöglicht einfache Erweiterung und Anpassung an individuelle Anforderungen.
- **Einfach zu bedienen**: Gut dokumentierte API mit Beispielen für einen schnellen Einstieg.

## Installation

Fügen Sie die `stellarsort_core`-Bibliothek zu Ihren `Cargo.toml`-Abhängigkeiten hinzu:

```toml
[dependencies]
stellarsort_core = "0.1.0"
```

Stellen Sie sicher, dass Sie die neueste Version auf [crates.io](https://crates.io/crates/stellarsort_core) überprüfen.

## Verwendung

Importieren Sie die Bibliothek in Ihrem Rust-Projekt und nutzen Sie die verfügbaren Funktionen zur Bildanalyse.

### Beispiel: Unscharfe Bilder erkennen

```rust
use stellarsort_core::detect_blur;
use image::DynamicImage;

fn main() {
    // Laden Sie ein Bild
    let img = image::open("beispielbild.png").expect("Bild konnte nicht geladen werden");

    // Definieren Sie den Schärfe-Schwellenwert und den Sigma-Wert für die Rauschunterdrückung
    let blur_threshold = 100.0;
    let denoise_sigma = 1.0;

    // Überprüfen Sie, ob das Bild unscharf ist
    if detect_blur(img, blur_threshold, denoise_sigma, None) {
        println!("Das Bild ist unscharf.");
    } else {
        println!("Das Bild ist scharf.");
    }
}
```

## API Referenz

Die vollständige API-Dokumentation finden Sie auf [docs.rs](https://docs.rs/stellarsort_core). Hier sind einige der Hauptfunktionen:

- `detect_blur`: Erkennt unscharfe Bilder basierend auf der Varianz der Laplacian-Ergebnisse.
- `calculate_variance`: Berechnet die Varianz der Pixelwerte in einem Graustufenbild.
- `calculate_noise`: Misst das Rauschniveau eines Bildes durch Berechnung der Standardabweichung.
- `analyze_images_parallel`: Führt die Bildanalyse parallel auf einer Liste von Bildern durch.

## Beitragen

Beiträge sind herzlich willkommen! Wenn Sie Fehler finden oder neue Features vorschlagen möchten:

1. Forken Sie das Repository
2. Erstellen Sie einen neuen Branch (`git checkout -b feature/neues-feature`)
3. Committen Sie Ihre Änderungen (`git commit -am 'Füge neues Feature hinzu'`)
4. Pushen Sie den Branch (`git push origin feature/neues-feature`)
5. Erstellen Sie einen Pull Request

Bitte stellen Sie sicher, dass Ihre Beiträge gut dokumentiert und getestet sind.

## Lizenz

Dieses Projekt steht unter der [MIT-Lizenz](LICENSE).

## Danksagungen

Ein herzliches Dankeschön an alle Open-Source-Projekte und Bibliotheken, die diese Arbeit ermöglicht haben:

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)
- [rayon](https://crates.io/crates/rayon)
- [thiserror](https://crates.io/crates/thiserror)
- [indicatif](https://crates.io/crates/indicatif)

---

<p align="center">
  Entwickelt mit ❤️ von <a href="https://github.com/yourusername">yourusername</a>
</p>
