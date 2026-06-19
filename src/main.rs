// 10 PRINT TAB(32);"HAMURABI"
// 20 PRINT TAB(15);"CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY"
// 30 PRINT:PRINT:PRINT
// 80 PRINT "TRY YOUR HAND AT GOVERNING ANCIENT SUMERIA"
// 90 PRINT "FOR A TEN-YEAR TERM OF OFFICE.":PRINT
// 95 D1=0: P1=0
// 100 Z=0: P=95:S=2800: H=3000: E=H-S
// 110 Y=3: A=H/Y: I=5: Q=1
// 210 D=0
// 215 PRINT:PRINT:PRINT "HAMURABI:  I BEG TO REPORT TO YOU,": Z=Z+1
// 217 PRINT "IN YEAR";Z;",";D;"PEOPLE STARVED,";I;"CAME TO THE CITY,"
// 218 P=P+I
// 227 IF Q>0 THEN 230
// 228 P=INT(P/2)
// 229 PRINT "A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED."
// 230 PRINT "POPULATION IS NOW";P
// 232 PRINT "THE CITY NOW OWNS ";A;"ACRES."
// 235 PRINT "YOU HARVESTED";Y;"BUSHELS PER ACRE."
// 250 PRINT "THE RATS ATE";E;"BUSHELS."
// 260 PRINT "YOU NOW HAVE ";S;"BUSHELS IN STORE.": PRINT
// 270 IF Z=11 THEN 860
// 310 C=INT(10*RND(1)): Y=C+17
// 312 PRINT "LAND IS TRADING AT";Y;"BUSHELS PER ACRE."
// 320 PRINT "HOW MANY ACRES DO YOU WISH TO BUY";
// 321 INPUT Q: IF Q<0 THEN 850
// 322 IF Y*Q<=S THEN 330
// 323 GOSUB 710
// 324 GOTO 320
// 330 IF Q=0 THEN 340
// 331 A=A+Q: S=S-Y*Q: C=0
// 334 GOTO 400
// 340 PRINT "HOW MANY ACRES DO YOU WISH TO SELL";
// 341 INPUT Q: IF Q<0 THEN 850
// 342 IF Q<A THEN 350
// 343 GOSUB 720
// 344 GOTO 340
// 350 A=A-Q: S=S+Y*Q: C=0
// 400 PRINT
// 410 PRINT "HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE";
// 411 INPUT Q
// 412 IF Q<0 THEN 850
// 418 REM *** TRYING TO USE MORE GRAIN THAN IS IN SILOS?
// 420 IF Q<=S THEN 430
// 421 GOSUB 710
// 422 GOTO 410
// 430 S=S-Q: C=1: PRINT
// 440 PRINT "HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED";
// 441 INPUT D: IF D=0 THEN 511
// 442 IF D<0 THEN 850
// 444 REM *** TRYING TO PLANT MORE ACRES THAN YOU OWN?
// 445 IF D<=A THEN 450
// 446 GOSUB 720
// 447 GOTO 440
// 449 REM *** ENOUGH GRAIN FOR SEED?
// 450 IF INT(D/2)<=S THEN 455
// 452 GOSUB 710
// 453 GOTO 440
// 454 REM *** ENOUGH PEOPLE TO TEND THE CROPS?
// 455 IF D<10*P THEN 510
// 460 PRINT "BUT YOU HAVE ONLY";P;"PEOPLE TO TEND THE FIELDS!  NOW THEN,"
// 470 GOTO 440
// 510 S=S-INT(D/2)
// 511 GOSUB 800
// 512 REM *** A BOUNTIFUL HARVEST!
// 515 Y=C: H=D*Y: E=0
// 521 GOSUB 800
// 522 IF INT(C/2)<>C/2 THEN 530
// 523 REM *** RATS ARE RUNNING WILD!!
// 525 E=INT(S/C)
// 530 S=S-E+H
// 531 GOSUB 800
// 532 REM *** LET'S HAVE SOME BABIES
// 533 I=INT(C*(20*A+S)/P/100+1)
// 539 REM *** HOW MANY PEOPLE HAD FULL TUMMIES?
// 540 C=INT(Q/20)
// 541 REM *** HORROS, A 15% CHANCE OF PLAGUE
// 542 Q=INT(10*(2*RND(1)-.3))
// 550 IF P<C THEN 210
// 551 REM *** STARVE ENOUGH FOR IMPEACHMENT?
// 552 D=P-C: IF D>.45*P THEN 560
// 553 P1=((Z-1)*P1+D*100/P)/Z
// 555 P=C: D1=D1+D: GOTO 215
// 560 PRINT: PRINT "YOU STARVED";D;"PEOPLE IN ONE YEAR!!!"
// 565 PRINT "DUE TO THIS EXTREME MISMANAGEMENT YOU HAVE NOT ONLY"
// 566 PRINT "BEEN IMPEACHED AND THROWN OUT OF OFFICE BUT YOU HAVE"
// 567 PRINT "ALSO BEEN DECLARED NATIONAL FINK!!!!": GOTO 990
// 710 PRINT "HAMURABI:  THINK AGAIN.  YOU HAVE ONLY"
// 711 PRINT S;"BUSHELS OF GRAIN.  NOW THEN,"
// 712 RETURN
// 720 PRINT "HAMURABI:  THINK AGAIN.  YOU OWN ONLY";A;"ACRES.  NOW THEN,"
// 730 RETURN
// 800 C=INT(RND(1)*5)+1
// 801 RETURN
// 850 PRINT: PRINT "HAMURABI:  I CANNOT DO WHAT YOU WISH."
// 855 PRINT "GET YOURSELF ANOTHER STEWARD!!!!!"
// 857 GOTO 990
// 860 PRINT "IN YOUR 10-YEAR TERM OF OFFICE,";P1;"PERCENT OF THE"
// 862 PRINT "POPULATION STARVED PER YEAR ON THE AVERAGE, I.E. A TOTAL OF"
// 865 PRINT D1;"PEOPLE DIED!!": L=A/P
// 870 PRINT "YOU STARTED WITH 10 ACRES PER PERSON AND ENDED WITH"
// 875 PRINT L;"ACRES PER PERSON.": PRINT
// 880 IF P1>33 THEN 565
// 885 IF L<7 THEN 565
// 890 IF P1>10 THEN 940
// 892 IF L<9 THEN 940
// 895 IF P1>3 THEN 960
// 896 IF L<10 THEN 960
// 900 PRINT "A FANTASTIC PERFORMANCE!!!  CHARLEMANGE, DISRAELI, AND"
// 905 PRINT "JEFFERSON COMBINED COULD NOT HAVE DONE BETTER!":GOTO 990
// 940 PRINT "YOUR HEAVY-HANDED PERFORMANCE SMACKS OF NERO AND IVAN IV."
// 945 PRINT "THE PEOPLE (REMIANING) FIND YOU AN UNPLEASANT RULER, AND,"
// 950 PRINT "FRANKLY, HATE YOUR GUTS!!":GOTO 990
// 960 PRINT "YOUR PERFORMANCE COULD HAVE BEEN SOMEWHAT BETTER, BUT"
// 965 PRINT "REALLY WASN'T TOO BAD AT ALL. ";INT(P*.8*RND(1));"PEOPLE"
// 970 PRINT "WOULD DEARLY LIKE TO SEE YOU ASSASSINATED BUT WE ALL HAVE OUR"
// 975 PRINT "TRIVIAL PROBLEMS."
// 990 PRINT: FOR N=1 TO 10: PRINT CHR$(7);: NEXT N
// 995 PRINT "SO LONG FOR NOW.": PRINT
// 999 END

use rand::RngExt;
use std::io::{self, Write};

const MAX_YEARS: u32 = 10;
const ACRES_PER_PERSON: u32 = 10;
const BUSHELS_PER_PERSON: u32 = 20;

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
fn player_quit() -> ! {
    println!();
    println!("HAMURABI:  I CANNOT DO WHAT YOU WISH.");
    println!("GET YOURSELF ANOTHER STEWARD!!!!!");
    farewell();
    std::process::exit(1);
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
            total_deaths: 0,
            avg_starvation_rate: 0.0,
            year: 0,
            population: 95,
            grain: 2800,
            rats_ate: 200,
            yield_per_acre: 3,
            acres: 1000,
            immigrants: 5,
            plague: false,
            starved: 0,
        }
    }

    fn buy_land(&mut self, acres: u32, price: u32) {
        self.acres += acres;
        self.grain-= acres * price;
    }

    fn sell_land(&mut self, acres: u32, price: u32) {
        self.acres -= acres;
        self.grain+= acres * price;
    }
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
            let n = read_int("HOW MANY ACRES DO YOU WISH TO BUY? ");
            if n < 0 {
                player_quit();
            }
            let n = n as u32;
            if price * n <= state.grain{
                state.buy_land(n, price);
                break n;
            }
            not_enough_grain(state.grain);
        };

        // ── Sell acres (lines 340-350) — only if nothing bought ──────────────
        if buy == 0 {
            loop {
                let n = read_int("HOW MANY ACRES DO YOU WISH TO SELL? ");
                if n < 0 {
                    player_quit();
                }
                let n = n as u32;
                if n <= state.acres {
                    state.sell_land(n, price);
                    break;
                }
                not_enough_acres(state.acres);
            }
        };

        // ── Feed people (lines 410-430) ───────────────────────────────────────
        println!();
        let food: u32 = loop {
            let n = read_int("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE? ");
            if n < 0 {
                player_quit();
            }
            let n = n as u32;
            if n <= state.grain{
                break n;
            }
            not_enough_grain(state.grain);
        };
        state.grain-= food;
        println!();

        // ── Plant seed (lines 440-510) ────────────────────────────────────────
        let planted: u32 = loop {
            let n = read_int("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED? ");
            if n == 0 {
                break 0;
            }
            if n < 0 {
                player_quit();
            }
            let n = n as u32;
            if n > state.acres {
                not_enough_acres(state.acres);
                continue;
            }
            if n / 2 > state.grain {
                not_enough_grain(state.grain);
                continue;
            }
            if n > ACRES_PER_PERSON * state.population {
                println!(
                    "BUT YOU HAVE ONLY {} PEOPLE TO TEND THE FIELDS!  NOW THEN,",
                    state.population
                );
                continue;
            }
            break n;
        };

        // plant 2 acres with one bushel... original used integer division,
        //let seed_cost = planted.div_ceil(2);
        let seed_cost = planted / 2;
        state.grain-= seed_cost;

        // ── Harvest (lines 511-530) ───────────────────────────────────────────
        state.yield_per_acre = rnd.random_range(1..=5);
        let harvest = planted * state.yield_per_acre;
        state.rats_ate = 0;

        let c2 = rnd.random_range(1..=5);
        if c2 % 2 == 0 {
            // even → rats
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
                return;
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
}
