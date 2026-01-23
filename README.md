# RustJack

TUI Blackjack game written in Rust. Simple, fast, and designed for local (network via webrtc WIP) play in the terminal against a dealer.

## Features
- Play against a computer dealer using classic rules
- Configurable rules: number of decks, starting bank, min/max bet
- Supports main actions: hit, stand, double, (split — if a pair)
- Persistent records and statistics
- Small dependencies, focused on terminal usability

## Installation

Via Homebrew:
 - Direct formula:
 	- `brew install johncuba/tap/rustjack/rustjack`

 - With tap:
 	- `brew tap johncuba/tap`
 	- `brew install rustjack`

Build and run from source:
 - `git clone https://github.com/johncuba/rustjack.git`
 - `cd rustjack`
 - `cargo run --release`

## Start
- To start a game: `rustjack`
