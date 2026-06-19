use std::io::{self, Write};

const MAX_YEARS: i64 = 10;
const BUSHELS_PER_PERSON: i64 = 20;

// ── I/O helpers ──────────────────────────────────────────────────────────────

/// Prompt and read a non-negative integer; re-prompts on bad input.
/// Returns the raw i64 (caller checks for negatives).
fn read_int(prompt: &str) -> i64 {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            std::process::exit(0);
        }
        if let Ok(n) = line.trim().parse::<i64>() {
            return n;
        }
        println!("PLEASE ENTER A WHOLE NUMBER.");
    }
}

fn not_enough_grain(s: i64) {
    println!("HAMURABI:  THINK AGAIN.  YOU HAVE ONLY");
    println!("{s} BUSHELS OF GRAIN.  NOW THEN,");
}

fn not_enough_acres(a: i64) {
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
fn impeach(d: i64, per_year: bool) {
    println!();
    if per_year {
        println!("YOU STARVED {} PEOPLE IN ONE YEAR!!!", d);
    }
    println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
    println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
    println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
}

/// Quit at the player's own request (line 850).
fn player_quit() {
    println!();
    println!("HAMURABI:  I CANNOT DO WHAT YOU WISH.");
    println!("GET YOURSELF ANOTHER STEWARD!!!!!");
    farewell();
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
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
    let mut total_deaths: i64 = 0; // cumulative deaths
    let mut avg_starvation_rate: f64 = 0.0; // running average % starved per year
    let mut year: i64 = 0; // year counter
    let mut population: i64 = 95;
    let mut store: i64 = 2800; // bushels in store
    let mut rats_ate: i64 = 200; // rats ate  (H-S = 3000-2800)
    let mut yield_per_acre: i64 = 3; // bushels harvested per acre
    let mut acres: i64 = 1000; // acres owned  (H/Y = 3000/3)
    let mut immigrants: i64 = 5; // immigrants last year
    let mut plague: bool = false;
    let mut starved: i64 = 0; // people who starved last year (shown in report)

    // ── Year loop ────────────────────────────────────────────────────────────
    loop {
        // ── Annual report (lines 215-260) ────────────────────────────────────
        println!();
        println!();
        println!("HAMURABI:  I BEG TO REPORT TO YOU,");
        year += 1;
        println!("IN YEAR {year}, {starved} PEOPLE STARVED, {immigrants} CAME TO THE CITY,");
        population += immigrants;

        // Plague (lines 227-229): q ≤ 0 means plague struck
        if plague {
            population /= 2;
            println!("A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED.");
        }

        println!("POPULATION IS NOW {}", population);
        println!("THE CITY NOW OWNS  {acres} ACRES.");
        println!("YOU HARVESTED {} BUSHELS PER ACRE.", yield_per_acre);
        println!("THE RATS ATE {rats_ate} BUSHELS.");
        println!("YOU NOW HAVE {store} BUSHELS IN STORE.");
        println!();

        // End of 10-year term (line 270)
        if year > MAX_YEARS {
            break;
        }

        // ── Land price (lines 310-312) ────────────────────────────────────────
        yield_per_acre = rand::random_range(17..=26);
        println!("LAND IS TRADING AT {} BUSHELS PER ACRE.", yield_per_acre);

        // ── Buy acres (lines 320-334) ─────────────────────────────────────────
        let buy: i64 = loop {
            let n = read_int("HOW MANY ACRES DO YOU WISH TO BUY? ");
            if n < 0 {
                return player_quit();
            }
            if yield_per_acre * n <= store {
                break n;
            }
            not_enough_grain(store);
        };

        // ── Sell acres (lines 340-350) — only if nothing bought ──────────────
        let sell: i64 = if buy == 0 {
            loop {
                let n = read_int("HOW MANY ACRES DO YOU WISH TO SELL? ");
                if n < 0 {
                    return player_quit();
                }
                if n < acres {
                    break n;
                }
                not_enough_acres(acres);
            }
        } else {
            0
        };

        acres += buy - sell;
        store += yield_per_acre * (sell - buy);

        // ── Feed people (lines 410-430) ───────────────────────────────────────
        println!();
        let food: i64 = loop {
            let n = read_int("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE? ");
            if n < 0 {
                return player_quit();
            }
            if n <= store {
                break n;
            }
            not_enough_grain(store);
        };
        store -= food;
        println!();

        // ── Plant seed (lines 440-510) ────────────────────────────────────────
        let planted: i64 = loop {
            let n = read_int("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED? ");
            if n == 0 {
                break 0;
            }
            if n < 0 {
                return player_quit();
            }
            if n > acres {
                not_enough_acres(acres);
                continue;
            }
            if n / 2 > store {
                not_enough_grain(store);
                continue;
            }
            if n >= 10 * population {
                println!(
                    "BUT YOU HAVE ONLY {} PEOPLE TO TEND THE FIELDS!  NOW THEN,",
                    population
                );
                continue;
            }
            break n;
        };

        store -= planted / 2; // seed cost

        // ── Harvest (lines 511-530) ───────────────────────────────────────────
        yield_per_acre = rand::random_range(1..=5);
        let harvest = planted * yield_per_acre;
        rats_ate = 0;

        let rat_roll = rand::random_range(1..=5);
        if rat_roll % 2 == 0 {
            // even → rats
            rats_ate = store / rat_roll;
        }
        store += harvest - rats_ate;

        // ── Immigration (line 533) ────────────────────────────────────────────
        let c3 = rand::random_range(1..=5); // chaos factor
        // immigrants proportional to attractiveness of city
        immigrants =
            (c3 as f64 * (BUSHELS_PER_PERSON * acres + store) as f64 / population as f64 / 100.0
                + 1.0)
                .floor() as i64;

        // ── Starvation (lines 540-555) ────────────────────────────────────────
        let fed_count = food / BUSHELS_PER_PERSON;

        plague = rand::random_bool(0.15);

        if population < fed_count {
            // Everyone fed; no starvation
            starved = 0;
        } else {
            starved = population - fed_count;
            if starved as f64 > 0.45 * population as f64 {
                impeach(starved, true);
                farewell();
                return;
            }
            // Running average of % starved
            avg_starvation_rate = ((year - 1) as f64 * avg_starvation_rate
                + starved as f64 * 100.0 / population as f64)
                / year as f64;
            population = fed_count;
            total_deaths += starved;
        }
    }

    // ── End-of-term evaluation (lines 860-975) ─────────────────────────────
    println!("IN YOUR {MAX_YEARS}-YEAR TERM OF OFFICE, {avg_starvation_rate:.2} PERCENT OF THE");
    println!("POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF");
    println!("{total_deaths} PEOPLE DIED!!");
    let l = acres / population;
    println!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH");
    println!("{l} ACRES PER PERSON.");
    println!();

    // Worst outcome: fink (lines 880-885 → 565)
    if avg_starvation_rate > 33.0 || l < 7 {
        impeach(0, false);
    } else if avg_starvation_rate > 10.0 || l < 9 {
        // Bad outcome (lines 890-892 → 940)
        println!("YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV.");
        println!("THE PEOPLE (REMAINING) FIND YOU AN UNPLEASANT RULER, AND,");
        println!("FRANKLY, HATE YOUR GUTS!!");
    } else if avg_starvation_rate > 3.0 || l < 10 {
        // Mediocre outcome (lines 895-896 → 960)
        let max = (0.8 * population as f64) as i64;
        let assassins = if max > 0 {
            rand::random_range(0..max)
        } else {
            0
        };
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
}
