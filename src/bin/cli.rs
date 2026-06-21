use hammurabi::{GameUi, MAX_YEARS, PlayerInput, UiMessage, Verdict, run_game};
use std::io::{self, Write};

struct Cli;

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn impeach(deaths: u32, per_year: bool) {
    println!();
    if per_year {
        println!("YOU STARVED {deaths} PEOPLE IN ONE YEAR!!!");
    }
    println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
    println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
    println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
}

impl GameUi for Cli {
    fn show(&mut self, message: UiMessage) -> io::Result<()> {
        match message {
            UiMessage::Intro => {
                // ── Title (lines 10-90) ──────────────────────────────────────────────────
                println!("{:>40}", "HAMURABI");
                println!("{:>51}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
                println!();
                println!();
                println!();
                println!("TRY YOUR HAND AT GOVERNING ANCIENT SUMERIA");
                println!("FOR A TEN-YEAR TERM OF OFFICE.");
                println!();
            }
            UiMessage::Report(state) => {
                clear_screen();
                print!("{state}");
            }
            UiMessage::LandPrice(n) => println!("LAND IS TRADING AT {n} BUSHELS PER ACRE."),
            UiMessage::NotEnoughGrain(n) => {
                println!("HAMURABI:  THINK AGAIN.  YOU HAVE ONLY");
                println!("{n} BUSHELS OF GRAIN.  NOW THEN,");
            }
            UiMessage::NotEnoughAcres(n) => {
                println!("HAMURABI:  THINK AGAIN.  YOU OWN ONLY {n} ACRES.  NOW THEN,")
            }
            UiMessage::NotEnoughWorkers(n) => {
                println!("BUT YOU HAVE ONLY {n} PEOPLE TO TEND THE FIELDS!  NOW THEN,")
            }
            UiMessage::Impeach { deaths, per_year } => impeach(deaths, per_year),
            UiMessage::EndOfTerm {
                state,
                acres_per_person,
                verdict,
            } => {
                println!(
                    "IN YOUR {MAX_YEARS}-YEAR TERM OF OFFICE, {:.2} PERCENT OF THE",
                    state.avg_starvation_rate
                );
                println!("POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF");
                println!("{} PEOPLE DIED!!", state.total_deaths);
                println!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH");
                println!("{acres_per_person} ACRES PER PERSON.");
                println!();

                match verdict {
                    Verdict::Worst => impeach(0, false),
                    Verdict::Bad => {
                        println!("YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV.");
                        println!("THE PEOPLE (REMAINING) FIND YOU AN UNPLEASANT RULER, AND,");
                        println!("FRANKLY, HATE YOUR GUTS!!");
                    }
                    Verdict::Mediocre(assassins) => {
                        println!("YOUR PERFORMANCE COULD HAVE BEEN SOMEWHAT BETTER, BUT");
                        println!("REALLY WASN'T TOO BAD AT ALL. {assassins} PEOPLE");
                        println!("WOULD DEARLY LIKE TO SEE YOU ASSASSINATED BUT WE ALL HAVE OUR");
                        println!("TRIVIAL PROBLEMS.");
                    }
                    Verdict::Best => {
                        println!("A FANTASTIC PERFORMANCE!!!  CHARLEMAGNE, DISRAELI, AND");
                        println!("JEFFERSON COMBINED COULD NOT HAVE DONE BETTER!");
                    }
                }
            }
            UiMessage::Quit => {
                println!();
                println!("\nHAMURABI:  I CANNOT DO WHAT YOU WISH.");
                println!("GET YOURSELF ANOTHER STEWARD!!!!!");
            }
            UiMessage::Farewell => {
                println!();
                // CHR$(7) × 10 — terminal bell
                for _ in 0..10 {
                    print!("\x07");
                }
                io::stdout().flush()?;
                println!("SO LONG FOR NOW.");
                println!();
            }
        }
        Ok(())
    }

    fn ask(&mut self, prompt: &str) -> io::Result<PlayerInput> {
        let mut line = String::new();
        loop {
            print!("{prompt}");
            io::stdout().flush()?;

            line.clear();
            if io::stdin().read_line(&mut line)? == 0 {
                return Ok(PlayerInput::Quit); // Handle EOF gracefully
            }

            if let Ok(n) = line.trim().parse::<i64>() {
                if n < 0 {
                    return Ok(PlayerInput::Quit);
                }
                return Ok(PlayerInput::Amount(n as u32));
            }
            println!("PLEASE ENTER A WHOLE NUMBER.");
        }
    }
}

fn main() -> io::Result<()> {
    let mut ui = Cli;
    run_game(&mut ui)
}
