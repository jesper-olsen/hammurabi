use rand::RngExt;
use std::io::{self, Write};

const MAX_YEARS: u32 = 10;
const ACRES_PER_PERSON: u32 = 10;
const BUSHELS_PER_PERSON: u32 = 20;

enum PlayerInput {
    Amount(u32),
    Quit,
}

// ── I/O helpers ──────────────────────────────────────────────────────────────

fn read_input(prompt: &str) -> io::Result<PlayerInput> {
    loop {
        print!("{prompt}");
        io::stdout().flush()?;

        let mut line = String::new();
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

fn not_enough_grain(s: u32) {
    println!("HAMURABI:  THINK AGAIN.  YOU HAVE ONLY");
    println!("{s} BUSHELS OF GRAIN.  NOW THEN,");
}

fn not_enough_acres(a: u32) {
    println!("HAMURABI:  THINK AGAIN.  YOU OWN ONLY {a} ACRES.  NOW THEN,");
}

// ── Endings ──────────────────────────────────────────────────────────────────

fn farewell() {
    println!();
    // CHR$(7) × 10 — terminal bell
    for _ in 0..10 {
        print!("\x07");
    }
    io::stdout().flush().unwrap();
    println!("SO LONG FOR NOW.");
    println!();
}

/// Instant impeachment (lines 560-567 / 850-857).
fn impeach(d: u32, per_year: bool) {
    println!();
    if per_year {
        println!("YOU STARVED {} PEOPLE IN ONE YEAR!!!", d);
    }
    println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
    println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
    println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
}

/// Quit at the player's own request (line 850).
fn quit_game() -> io::Result<()> {
    println!();
    println!("\nHAMURABI:  I CANNOT DO WHAT YOU WISH.");
    println!("GET YOURSELF ANOTHER STEWARD!!!!!");
    farewell();
    Ok(())
}

struct State {
    total_deaths: u32,        // cumulative deaths
    avg_starvation_rate: f64, // running average % starved per year
    year: u32,                // year counter
    population: u32,
    grain: u32,          // bushels in store
    rats_ate: u32,       // rats ate  (H-S = 3000-2800)
    yield_per_acre: u32, // bushels harvested per acre
    acres: u32,          // acres owned  (H/Y = 3000/3)
    immigrants: u32,     // immigrants last year
    plague: bool,
    starved: u32, // people who starved last year (shown in report)
}

impl State {
    fn new() -> Self {
        State {
            year: 0,
            population: 95,
            grain: 2800,
            acres: 1000,

            // (last) year record
            total_deaths: 0,
            avg_starvation_rate: 0.0,
            rats_ate: 200,
            yield_per_acre: 3,
            immigrants: 5,
            plague: false,
            starved: 0,
        }
    }

    fn buy_land(&mut self, acres: u32, price: u32) {
        self.acres += acres;
        self.grain -= acres * price;
    }

    fn sell_land(&mut self, acres: u32, price: u32) {
        self.acres -= acres;
        self.grain += acres * price;
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    // ── Title (lines 10-90) ──────────────────────────────────────────────────
    println!("{:>40}", "HAMURABI");
    println!("{:>51}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
    println!();
    println!();
    println!();
    println!("TRY YOUR HAND AT GOVERNING ANCIENT SUMERIA");
    println!("FOR A TEN-YEAR TERM OF OFFICE.");
    println!();

    // ── Initial state (lines 95-110) ─────────────────────────────────────────
    let mut state = State::new();
    let mut rnd = rand::rng();

    // ── Year loop ────────────────────────────────────────────────────────────
    loop {
        state.year += 1;

        // ── Annual report (lines 215-260) ────────────────────────────────────
        println!();
        println!();
        println!("HAMURABI:  I BEG TO REPORT TO YOU,");
        println!(
            "IN YEAR {}, {} PEOPLE STARVED, {} CAME TO THE CITY,",
            state.year, state.starved, state.immigrants
        );
        state.population += state.immigrants;

        // Plague (lines 227-229): q ≤ 0 means plague struck
        if state.plague {
            state.population /= 2;
            println!("A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED.");
        }

        println!("POPULATION IS NOW {}", state.population);
        println!("THE CITY NOW OWNS  {} ACRES.", state.acres);
        println!("YOU HARVESTED {} BUSHELS PER ACRE.", state.yield_per_acre);
        println!("THE RATS ATE {} BUSHELS.", state.rats_ate);
        println!("YOU NOW HAVE {} BUSHELS IN STORE.", state.grain);
        println!();

        // End of 10-year term (line 270)
        if state.year > MAX_YEARS {
            break;
        }

        // ── Land price (lines 310-312) ────────────────────────────────────────
        let price = rnd.random_range(17..=26);
        println!("LAND IS TRADING AT {} BUSHELS PER ACRE.", price);

        // ── Buy acres (lines 320-334) ─────────────────────────────────────────
        let buy: u32 = loop {
            match read_input("HOW MANY ACRES DO YOU WISH TO BUY? ")? {
                PlayerInput::Quit => return quit_game(),
                PlayerInput::Amount(n) if price * n <= state.grain => {
                    state.buy_land(n, price);
                    break n;
                }
                PlayerInput::Amount(_) => not_enough_grain(state.grain),
            }
        };

        // ── Sell acres (lines 340-350) — only if nothing bought ──────────────
        if buy == 0 {
            loop {
                match read_input("HOW MANY ACRES DO YOU WISH TO SELL? ")? {
                    PlayerInput::Quit => return quit_game(),
                    PlayerInput::Amount(n) if n <= state.acres => {
                        state.sell_land(n, price);
                        break;
                    }
                    PlayerInput::Amount(_) => not_enough_acres(state.acres),
                }
            }
        };

        // ── Feed people (lines 410-430) ───────────────────────────────────────
        println!();
        let food: u32 = loop {
            match read_input("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE? ")? {
                PlayerInput::Quit => return quit_game(),
                PlayerInput::Amount(n) if n <= state.grain => break n,
                PlayerInput::Amount(_) => not_enough_grain(state.grain),
            }
        };
        state.grain -= food;
        println!();

        // ── Plant seed (lines 440-510) ────────────────────────────────────────
        let planted: u32 = loop {
            match read_input("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED? ")? {
                PlayerInput::Quit => return quit_game(),
                PlayerInput::Amount(0) => break 0,
                PlayerInput::Amount(n) if n > state.acres => not_enough_acres(state.acres),
                PlayerInput::Amount(n) if n / 2 > state.grain => not_enough_grain(state.grain),
                PlayerInput::Amount(n) if n > ACRES_PER_PERSON * state.population => {
                    println!(
                        "BUT YOU HAVE ONLY {} PEOPLE TO TEND THE FIELDS!  NOW THEN,",
                        state.population
                    );
                }
                PlayerInput::Amount(n) => break n,
            }
        };

        // plant 2 acres with one bushel... original used integer division,
        //let seed_cost = planted.div_ceil(2);
        let seed_cost = planted / 2;
        state.grain -= seed_cost;

        // ── Harvest (lines 511-530) ───────────────────────────────────────────
        state.yield_per_acre = rnd.random_range(1..=5);
        let harvest = planted * state.yield_per_acre;
        state.rats_ate = 0;

        let c2 = rnd.random_range(1..=5);
        if c2 % 2 == 0 {
            // even => rats
            state.rats_ate = state.grain / c2;
        }
        state.grain += harvest - state.rats_ate;

        // ── Immigration (line 533) ────────────────────────────────────────────
        //let c3 = rnd.random_range(1..=5); // chaos factor
        // immigrants proportional to attractiveness of city
        // Uses c2, the same roll as the rat check above - not a fresh draw
        state.immigrants = (c2 as f64 * (BUSHELS_PER_PERSON * state.acres + state.grain) as f64
            / state.population as f64
            / 100.0
            + 1.0)
            .floor() as u32;

        // ── Starvation (lines 540-555) ────────────────────────────────────────
        let fed_count = food / BUSHELS_PER_PERSON;

        state.plague = rnd.random_bool(0.15);

        if state.population < fed_count {
            // Everyone fed; no starvation
            state.starved = 0;
        } else {
            state.starved = state.population - fed_count;
            if state.starved as f64 > 0.45 * state.population as f64 {
                impeach(state.starved, true);
                farewell();
                return Ok(());
            }
            // Running average of % starved
            state.avg_starvation_rate = ((state.year - 1) as f64 * state.avg_starvation_rate
                + state.starved as f64 * 100.0 / state.population as f64)
                / state.year as f64;
            state.population = fed_count;
            state.total_deaths += state.starved;
        }
    }

    // ── End-of-term evaluation (lines 860-975) ─────────────────────────────
    println!(
        "IN YOUR {MAX_YEARS}-YEAR TERM OF OFFICE, {:.2} PERCENT OF THE",
        state.avg_starvation_rate
    );
    println!("POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF");
    println!("{} PEOPLE DIED!!", state.total_deaths);
    let l = state.acres / state.population;
    println!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH");
    println!("{l} ACRES PER PERSON.");
    println!();

    if state.avg_starvation_rate > 33.0 || l < 7 {
        // Worst outcome: fink (lines 880-885 → 565)
        impeach(0, false);
    } else if state.avg_starvation_rate > 10.0 || l < 9 {
        // Bad outcome (lines 890-892 → 940)
        println!("YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV.");
        println!("THE PEOPLE (REMAINING) FIND YOU AN UNPLEASANT RULER, AND,");
        println!("FRANKLY, HATE YOUR GUTS!!");
    } else if state.avg_starvation_rate > 3.0 || l < 10 {
        // Mediocre outcome (lines 895-896 → 960)
        let max = (0.8 * state.population as f64) as u32;
        let assassins = if max > 0 { rnd.random_range(0..max) } else { 0 };
        println!("YOUR PERFORMANCE COULD HAVE BEEN SOMEWHAT BETTER, BUT");
        println!("REALLY WASN'T TOO BAD AT ALL. {assassins} PEOPLE");
        println!("WOULD DEARLY LIKE TO SEE YOU ASSASSINATED BUT WE ALL HAVE OUR");
        println!("TRIVIAL PROBLEMS.");
    } else {
        // Best outcome (lines 900-905)
        println!("A FANTASTIC PERFORMANCE!!!  CHARLEMAGNE, DISRAELI, AND");
        println!("JEFFERSON COMBINED COULD NOT HAVE DONE BETTER!");
    }
    farewell();
    Ok(())
}
