use std::collections::HashSet;
use std::fs::File;
use std::io::{self,BufReader, Write};
use serde::Deserialize;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    options: [String; 4],
    answer: String,
    difficulty: String, // "easy", "medium", "hard"
    explanation: Option<String>
}

fn quest(filename: &str) -> Vec<Question> {
    let file = File::open(filename).expect("Failed to open questions file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}

fn get_score(difficulty: &str) -> i32 {
    match difficulty {
        "easy" => 10,
        "medium" => 20,
        "hard" => 30,
        _ => 0
    }
}

fn grade(percent: f64) -> &'static str {
    match percent {
        90.0..=100.0 => "🏆 Ballon d'Or Worthy",
        70.0..=89.9 => "⭐ World-Class",
        50.0..=69.9 => "⚽ Mid-table Performer",
        30.0..=49.9 => "🪑 Bench Warmer",
        _ => "📉 Relegation Worthy"
    }
}

fn in_time(timeout_secs: u64) -> Option<String> {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(timeout_secs) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    return Some(c.to_ascii_uppercase().to_string());
                }
            }
        }
    }
    None
}

fn r_cont() {
    println!("Type 'R' to move to the next question.");
    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    if c.eq_ignore_ascii_case(&'r') {
                        break;
                    } else {
                        println!("Please type 'R' to continue.");
                    }
                }
            }
        }
    }
}

// Read line from stdin
fn read_in() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let questions = quest("questions.json");

    println!("👋 Welcome to the ⚽ Football Quiz!");
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    let user_name = read_in();

    println!("Hello, {}! Let's start the quiz.\n", user_name);

    // ask kora questions er set
    let mut askQ = HashSet::new();

    loop {
        // ask kora questions er set theke notun questions ber kora
        let mut easy: Vec<_> = questions.iter().filter(|q| q.difficulty == "easy" && !askQ.contains(&q.question)).collect();
        let mut medium: Vec<_> = questions.iter().filter(|q| q.difficulty == "medium" && !askQ.contains(&q.question)).collect();
        let mut hard: Vec<_> = questions.iter().filter(|q| q.difficulty == "hard" && !askQ.contains(&q.question)).collect();

        let mut rng = rand::rng();
        easy.shuffle(&mut rng);
        medium.shuffle(&mut rng);
        hard.shuffle(&mut rng);

        // question nai? bye bye
        if easy.len() < 4 || medium.len() < 3 || hard.len() < 3 {
            println!("No more new questions available. Thanks for playing, {}!", user_name);
            break;
        }

        let selectQ: Vec<_> = easy.into_iter().take(4).chain(medium.into_iter().take(3)).chain(hard.into_iter().take(3)).collect();

        let total_possible: i32 = selectQ.iter().map(|q| get_score(&q.difficulty)).sum();

        let mut earned_points = 0;

        println!("🎮 Starting a new round! You have 10 seconds per question.\n");

        for (i, q) in selectQ.iter().enumerate() {
            println!("Q{} ({}): {}", i + 1, q.difficulty.to_uppercase(), q.question);
            for option in &q.options {
                println!("{}", option);
            }

            print!("⏱️ Answer (A/B/C/D): ");
            io::stdout().flush().unwrap();

            let answer = in_time(10);

            match answer {
                Some(ans) if ans == q.answer => {
                    println!("✅ Correct!\n");
                    earned_points += get_score(&q.difficulty);
                }
                Some(ans) => {
                    println!("❌ Wrong! You said {} | Correct: {}\n", ans, q.answer);
                    if let Some(exp) = &q.explanation {
                        println!("💡 Explanation: {}\n", exp);
                    }
                    r_cont();
                }
                None => {
                    println!("⏰ Time's up! Correct: {}\n", q.answer);
                    if let Some(exp) = &q.explanation {
                        println!("💡 Explanation: {}\n", exp);
                    }
                    r_cont();
                }
            }
            askQ.insert(q.question.clone());
        }

        let percentage = (earned_points as f64 / total_possible as f64) * 100.0;
        println!("📊 Round Score: {:.1}%", percentage);
        println!("🏅 Grade: {}\n", grade(percentage));

        // Ask if user wants to continue
        print!("Do you want to continue? (Y/N): ");
        io::stdout().flush().unwrap();
        let cont = read_in();

        if !cont.eq_ignore_ascii_case("y") {
            println!("Thanks for playing, {user_name}! Goodbye!");
            break;
        }
    }
}
//Absolute Cinema
// Now I like Rust-O-Vasha