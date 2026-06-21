use hammurabi::{GameUi, MAX_YEARS, PlayerInput, State, Verdict, run_game};
use std::io::{self, Write};

struct Cli;

impl GameUi for Cli {
    fn intro(&mut self) -> io::Result<()> {
        // ── Title (lines 10-90) ──────────────────────────────────────────────────
        println!("{:>40}", "HAMURABI");
        println!("{:>51}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
        println!();
        println!();
        println!();
        println!("TRY YOUR HAND AT GOVERNING ANCIENT SUMERIA");
        println!("FOR A TEN-YEAR TERM OF OFFICE.");
        println!();
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

    fn render_report(&mut self, state: &State) -> io::Result<()> {
        print!("{state}");
        Ok(())
    }

    fn not_enough_grain(&mut self, s: u32) -> io::Result<()> {
        println!("HAMURABI:  THINK AGAIN.  YOU HAVE ONLY");
        println!("{s} BUSHELS OF GRAIN.  NOW THEN,");
        Ok(())
    }

    fn not_enough_acres(&mut self, a: u32) -> io::Result<()> {
        println!("HAMURABI:  THINK AGAIN.  YOU OWN ONLY {a} ACRES.  NOW THEN,");
        Ok(())
    }

    fn report_land_price(&mut self, price: u32) -> io::Result<()> {
        println!("LAND IS TRADING AT {price} BUSHELS PER ACRE.");
        Ok(())
    }

    fn not_enough_workers(&mut self, population: u32) -> io::Result<()> {
        println!("BUT YOU HAVE ONLY {population} PEOPLE TO TEND THE FIELDS!  NOW THEN,");
        Ok(())
    }

    // ── Endings ──────────────────────────────────────────────────────────────────

    fn farewell(&mut self) -> io::Result<()> {
        println!();
        // CHR$(7) × 10 — terminal bell
        for _ in 0..10 {
            print!("\x07");
        }
        io::stdout().flush()?;
        println!("SO LONG FOR NOW.");
        println!();
        Ok(())
    }

    /// Instant impeachment (lines 560-567 / 850-857).
    fn impeach(&mut self, d: u32, per_year: bool) -> io::Result<()> {
        println!();
        if per_year {
            println!("YOU STARVED {} PEOPLE IN ONE YEAR!!!", d);
        }
        println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
        println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
        println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
        Ok(())
    }

    /// Quit at the player's own request (line 850).
    fn quit_game(&mut self) -> io::Result<()> {
        println!();
        println!("\nHAMURABI:  I CANNOT DO WHAT YOU WISH.");
        println!("GET YOURSELF ANOTHER STEWARD!!!!!");
        self.farewell()
    }

    fn end_of_term_eval(
        &mut self,
        state: &State,
        acres_per_person: u32,
        verdict: Verdict,
    ) -> io::Result<()> {
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
            Verdict::Worst => self.impeach(0, false)?,
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
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut ui = Cli;
    run_game(&mut ui)
}
