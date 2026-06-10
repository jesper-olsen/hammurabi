use std::io::{self, Write};

// ── RNG ──────────────────────────────────────────────────────────────────────
// Xorshift64 seeded from the system clock; no external crates needed.
fn rnd() -> f64 {
    use std::cell::Cell;
    use std::time::SystemTime;
    thread_local! {
        static SEED: Cell<u64> = Cell::new(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64 ^ 0xdeadbeef_cafebabe)
                .unwrap_or(6_364_136_223_846_793_005)
        );
    }
    SEED.with(|cell| {
        let mut x = cell.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        cell.set(x);
        // Map to [0, 1)
        (x >> 11) as f64 / (1u64 << 53) as f64
    })
}

/// BASIC's INT(RND(1)*5)+1 — yields 1..=5
fn rand_c() -> i64 {
    (rnd() * 5.0).floor() as i64 + 1
}

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
        println!("YOU STARVED {d} PEOPLE IN ONE YEAR!!!", d);
    }
    println!("DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY");
    println!("BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE");
    println!("ALSO BEEN DECLARED NATIONAL FINK!!!!");
    farewell();
    std::process::exit(0);
}

/// Quit at the player's own request (line 850).
fn player_quit() {
    println!();
    println!("HAMURABI:  I CANNOT DO WHAT YOU WISH.");
    println!("GET YOURSELF ANOTHER STEWARD!!!!!");
    farewell();
    std::process::exit(0);
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
    let mut d1: i64 = 0; // cumulative deaths
    let mut p1: f64 = 0.0; // running average % starved per year
    let mut z: i64 = 0; // year counter
    let mut p: i64 = 95; // population
    let mut s: i64 = 2800; // bushels in store
    let mut e: i64 = 200; // rats ate  (H-S = 3000-2800)
    let mut y: i64 = 3; // bushels harvested per acre
    let mut a: i64 = 1000; // acres owned  (H/Y = 3000/3)
    let mut i_imm: i64 = 5; // immigrants last year
    let mut q: i64 = 1; // plague flag: >0 = no plague
    let mut d: i64 = 0; // people who starved last year (shown in report)

    // ── Year loop ────────────────────────────────────────────────────────────
    loop {
        // ── Annual report (lines 215-260) ────────────────────────────────────
        println!();
        println!();
        println!("HAMURABI:  I BEG TO REPORT TO YOU,");
        z += 1;
        println!("IN YEAR {z}, {d} PEOPLE STARVED, {i_imm} CAME TO THE CITY,");
        p += i_imm;

        // Plague (lines 227-229): q ≤ 0 means plague struck
        if q <= 0 {
            p /= 2;
            println!("A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED.");
        }

        println!("POPULATION IS NOW {p}");
        println!("THE CITY NOW OWNS  {a} ACRES.");
        println!("YOU HARVESTED {y} BUSHELS PER ACRE.");
        println!("THE RATS ATE {e} BUSHELS.");
        println!("YOU NOW HAVE  {s} BUSHELS IN STORE.");
        println!();

        // End of 10-year term (line 270)
        if z == 11 {
            break;
        }

        // ── Land price (lines 310-312) ────────────────────────────────────────
        y = (10.0 * rnd()).floor() as i64 + 17;
        println!("LAND IS TRADING AT {} BUSHELS PER ACRE.", y);

        // ── Buy acres (lines 320-334) ─────────────────────────────────────────
        let buy: i64 = loop {
            let n = read_int("HOW MANY ACRES DO YOU WISH TO BUY? ");
            if n < 0 {
                player_quit();
            }
            if y * n <= s {
                break n;
            }
            not_enough_grain(s);
        };

        // ── Sell acres (lines 340-350) — only if nothing bought ──────────────
        let sell: i64 = if buy == 0 {
            loop {
                let n = read_int("HOW MANY ACRES DO YOU WISH TO SELL? ");
                if n < 0 {
                    player_quit();
                }
                if n < a {
                    break n;
                }
                not_enough_acres(a);
            }
        } else {
            0
        };

        a += buy - sell;
        s -= y * buy;
        s += y * sell;

        // ── Feed people (lines 410-430) ───────────────────────────────────────
        println!();
        let food: i64 = loop {
            let n = read_int("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE? ");
            if n < 0 {
                player_quit();
            }
            if n <= s {
                break n;
            }
            not_enough_grain(s);
        };
        s -= food;
        println!();

        // ── Plant seed (lines 440-510) ────────────────────────────────────────
        let planted: i64 = loop {
            let n = read_int("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED? ");
            if n == 0 {
                break 0;
            }
            if n < 0 {
                player_quit();
            }
            if n > a {
                not_enough_acres(a);
                continue;
            }
            if n / 2 > s {
                not_enough_grain(s);
                continue;
            }
            if n >= 10 * p {
                println!(
                    "BUT YOU HAVE ONLY {} PEOPLE TO TEND THE FIELDS!  NOW THEN,",
                    p
                );
                continue;
            }
            break n;
        };

        s -= planted / 2; // seed cost

        // ── Harvest (lines 511-530) ───────────────────────────────────────────
        let yield_per_acre = rand_c(); // 1..=5
        y = yield_per_acre;
        let harvest = planted * y;
        e = 0;

        let rat_roll = rand_c(); // 1..=5
        if rat_roll % 2 == 0 {
            // even → rats
            e = s / rat_roll;
        }
        s = s - e + harvest;

        // ── Immigration (line 533) ────────────────────────────────────────────
        let c3 = rand_c();
        i_imm = (c3 as f64 * (20 * a + s) as f64 / p as f64 / 100.0 + 1.0).floor() as i64;

        // ── Starvation (lines 540-555) ────────────────────────────────────────
        let fed_count = food / 20; // each 20 bushels feeds one person

        // Plague roll for *next* year: >0 = no plague
        q = (10.0 * (2.0 * rnd() - 0.3)).floor() as i64;

        if p < fed_count {
            // Everyone fed; no starvation
            d = 0;
        } else {
            d = p - fed_count;
            if d as f64 > 0.45 * p as f64 {
                impeach(d, true);
            }
            // Running average of % starved
            p1 = ((z - 1) as f64 * p1 + d as f64 * 100.0 / p as f64) / z as f64;
            p = fed_count;
            d1 += d;
        }
    }

    // ── End-of-term evaluation (lines 860-975) ─────────────────────────────
    println!("IN YOUR 10-YEAR TERM OF OFFICE, {p1:.2} PERCENT OF THE");
    println!("POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF");
    println!("{d1} PEOPLE DIED!!");
    let l = a / p;
    println!("YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH");
    println!("{l} ACRES PER PERSON.");
    println!();

    // Worst outcome: fink (lines 880-885 → 565)
    if p1 > 33.0 || l < 7 {
        impeach(0, false);
    }

    // Bad outcome (lines 890-892 → 940)
    if p1 > 10.0 || l < 9 {
        println!("YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV.");
        println!("THE PEOPLE (REMAINING) FIND YOU AN UNPLEASANT RULER, AND,");
        println!("FRANKLY, HATE YOUR GUTS!!");
        farewell();
        return;
    }

    // Mediocre outcome (lines 895-896 → 960)
    if p1 > 3.0 || l < 10 {
        let assassins = (p as f64 * 0.8 * rnd()).floor() as i64;
        println!("YOUR PERFORMANCE COULD HAVE BEEN SOMEWHAT BETTER, BUT");
        println!("REALLY WASN'T TOO BAD AT ALL. {assassins} PEOPLE");
        println!("WOULD DEARLY LIKE TO SEE YOU ASSASSINATED BUT WE ALL HAVE OUR");
        println!("TRIVIAL PROBLEMS.");
        farewell();
        return;
    }

    // Best outcome (lines 900-905)
    println!("A FANTASTIC PERFORMANCE!!!  CHARLEMAGNE, DISRAELI, AND");
    println!("JEFFERSON COMBINED COULD NOT HAVE DONE BETTER!");
    farewell();
}
