# Football Quiz CLI (Rust)

A command-line football quiz game written in Rust. The quiz features questions from football history, including recent events up to the 2025 UEFA Champions League final.

## Features
- Multiple-choice football trivia questions
- Questions cover World Cups, UEFA Champions League, Ballon d'Or, and more
- Difficulty levels: easy, medium, hard
- Questions stored in `questions.json`

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable)

### Build and Run
```bash
cargo run
```

## Project Structure
- `src/main.rs` - Main application logic
- `questions.json` - Football quiz questions
- `Cargo.toml` - Rust dependencies and metadata

## Customizing Questions
Edit `questions.json` to add, remove, or update quiz questions. Each question includes options, the correct answer, and a difficulty level.

**Note:** The `questions.json` file was initially created by Grok3. Anyone can contribute by editing or adding new questions to the JSON file.

## License
MIT
