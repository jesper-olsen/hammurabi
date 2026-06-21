use rand::RngExt;
use std::fmt;
use std::io;

pub const MAX_YEARS: u32 = 10;
const ACRES_PER_PERSON: u32 = 10;
const BUSHELS_PER_PERSON: u32 = 20;

pub enum PlayerInput {
    Amount(u32),
    Quit,
}

enum YearOutcome {
    Continue,
    Impeached,
}

enum NotEnough {
    Grain,
    Acres,
    Workers,
}

pub enum Verdict {
    Worst,
    Bad,
    Mediocre(u32),
    Best,
}

pub trait GameUi {
    fn intro(&mut self) -> io::Result<()>;
    fn ask(&mut self, prompt: &str) -> io::Result<PlayerInput>;
    fn render_report(&mut self, state: &State) -> io::Result<()>;
    fn not_enough_grain(&mut self, amount: u32) -> io::Result<()>;
    fn not_enough_acres(&mut self, amount: u32) -> io::Result<()>;
    fn not_enough_workers(&mut self, population: u32) -> io::Result<()>;
    fn report_land_price(&mut self, price: u32) -> io::Result<()>;
    fn impeach(&mut self, deaths: u32, per_year: bool) -> io::Result<()>;
    fn quit_game(&mut self) -> io::Result<()>;
    fn end_of_term_eval(
        &mut self,
        state: &State,
        acres_per_person: u32,
        verdict: Verdict,
    ) -> io::Result<()>;
    fn farewell(&mut self) -> io::Result<()>;
}

pub struct State {
    pub total_deaths: u32,        // cumulative deaths
    pub avg_starvation_rate: f64, // running average % starved per year
    year: u32,                    // year counter
    population: u32,
    grain: u32,          // bushels in store
    rats_ate: u32,       // rats ate  (H-S = 3000-2800)
    yield_per_acre: u32, // bushels harvested per acre
    acres: u32,          // acres owned  (H/Y = 3000/3)
    immigrants: u32,     // immigrants last year
    plague: bool,
    starved: u32, // people who starved last year (shown in report)
    rng: rand::rngs::ThreadRng,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ── Annual report (lines 215-260) ────────────────────────────────────
        writeln!(f, "\n\nHAMURABI:  I BEG TO REPORT TO YOU,")?;
        writeln!(
            f,
            "IN YEAR {}, {} PEOPLE STARVED, {} CAME TO THE CITY,",
            self.year, self.starved, self.immigrants
        )?;
        // Plague (lines 227-229)
        if self.plague {
            writeln!(f, "A HORRIBLE PLAGUE STRUCK!  HALF THE PEOPLE DIED.")?;
        }
        writeln!(f, "POPULATION IS NOW {}", self.population)?;
        writeln!(f, "THE CITY NOW OWNS  {} ACRES.", self.acres)?;
        writeln!(f, "YOU HARVESTED {} BUSHELS PER ACRE.", self.yield_per_acre)?;
        writeln!(f, "THE RATS ATE {} BUSHELS.", self.rats_ate)?;
        writeln!(f, "YOU NOW HAVE {} BUSHELS IN STORE.", self.grain)?;
        writeln!(f)
    }
}

impl State {
    fn new() -> Self {
        // (lines 95-110)
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
            rng: rand::rng(),
        }
    }

    fn buy_land(&mut self, acres: u32, price: u32) -> Result<(), NotEnough> {
        if u64::from(price) * u64::from(acres) <= self.grain as u64 {
            self.acres += acres;
            self.grain -= acres * price;
            Ok(())
        } else {
            Err(NotEnough::Grain)
        }
    }

    fn sell_land(&mut self, acres: u32, price: u32) -> Result<(), NotEnough> {
        if acres <= self.acres {
            self.acres -= acres;
            self.grain += acres * price;
            Ok(())
        } else {
            Err(NotEnough::Acres)
        }
    }

    fn allocate_food(&mut self, bushels: u32) -> Result<(), NotEnough> {
        if bushels <= self.grain {
            self.grain -= bushels;
            Ok(())
        } else {
            Err(NotEnough::Grain)
        }
    }

    fn plant_seed(&mut self, acres: u32) -> Result<(), NotEnough> {
        // plant 2 acres with one bushel... original used integer division,
        // which means you can get free seed, e.g. plant 1 acre, cost is 0...
        //let seed_cost = acres / 2;
        let seed_cost = acres.div_ceil(2);
        if acres > self.acres {
            Err(NotEnough::Acres)
        } else if seed_cost > self.grain {
            Err(NotEnough::Grain)
        } else if acres > ACRES_PER_PERSON * self.population {
            Err(NotEnough::Workers)
        } else {
            self.grain -= seed_cost;
            Ok(())
        }
    }

    fn start_year(&mut self) {
        self.year += 1;
        self.population += self.immigrants;
        if self.plague {
            self.population /= 2;
        }
    }

    fn end_year(&mut self, food: u32, planted: u32) -> YearOutcome {
        // ── Harvest (lines 511-530) ───────────────────────────────────────────
        self.yield_per_acre = self.rng.random_range(1..=5);
        let harvest = planted * self.yield_per_acre;
        self.rats_ate = 0;

        let c2 = self.rng.random_range(1..=5);
        if c2 % 2 == 0 {
            // even => rats
            self.rats_ate = self.grain / c2;
        }
        self.grain += harvest - self.rats_ate;

        // ── Immigration (line 533) ────────────────────────────────────────────
        //let c3 = rnd.random_range(1..=5); // chaos factor
        // immigrants proportional to attractiveness of city
        // Uses c2, the same roll as the rat check above - not a fresh draw
        self.immigrants = (c2 as f64 * (BUSHELS_PER_PERSON * self.acres + self.grain) as f64
            / (self.population.max(1)) as f64
            / 100.0
            + 1.0)
            .floor() as u32;

        // ── Starvation (lines 540-555) ────────────────────────────────────────
        let fed_count = food / BUSHELS_PER_PERSON;

        self.plague = self.rng.random_bool(0.15);

        if fed_count >= self.population {
            // Everyone fed; no starvation
            self.starved = 0;
        } else {
            self.starved = self.population - fed_count;
            if self.starved as f64 > 0.45 * self.population as f64 {
                return YearOutcome::Impeached;
            }
            // Running average of % starved
            self.avg_starvation_rate = ((self.year - 1) as f64 * self.avg_starvation_rate
                + self.starved as f64 * 100.0 / self.population as f64)
                / self.year as f64;
            self.population = fed_count;
            self.total_deaths += self.starved;
        }
        YearOutcome::Continue
    }

    fn roll_land_price(&mut self) -> u32 {
        self.rng.random_range(17..=26)
    }

    fn assassins_roll(&mut self) -> u32 {
        let max = (0.8 * self.population as f64) as u32;
        if max > 0 {
            self.rng.random_range(0..max)
        } else {
            0
        }
    }
}

// -- main game loop --------------------------------------------------------

pub fn run_game<U: GameUi>(ui: &mut U) -> io::Result<()> {
    let mut state = State::new();
    ui.intro()?;

    // ── Year loop ────────────────────────────────────────────────────────────
    loop {
        state.start_year();
        ui.render_report(&state)?;

        // End of 10-year term (line 270)
        if state.year > MAX_YEARS {
            break;
        }

        // ── Land price (lines 310-312) ────────────────────────────────────────
        let price = state.roll_land_price();
        ui.report_land_price(price)?;

        // ── Buy acres (lines 320-334) ─────────────────────────────────────────
        let buy: u32 = loop {
            match ui.ask("HOW MANY ACRES DO YOU WISH TO BUY? ")? {
                PlayerInput::Quit => return ui.quit_game(),
                PlayerInput::Amount(n) => match state.buy_land(n, price) {
                    Ok(()) => break n,
                    Err(NotEnough::Grain) => ui.not_enough_grain(state.grain)?,
                    _ => unreachable!(),
                },
            }
        };

        // ── Sell acres (lines 340-350) — only if nothing bought ──────────────
        if buy == 0 {
            loop {
                match ui.ask("HOW MANY ACRES DO YOU WISH TO SELL? ")? {
                    PlayerInput::Quit => return ui.quit_game(),
                    PlayerInput::Amount(n) => match state.sell_land(n, price) {
                        Ok(()) => break,
                        Err(NotEnough::Acres) => ui.not_enough_acres(state.acres)?,
                        _ => unreachable!(),
                    },
                }
            }
        };

        // ── Feed people (lines 410-430) ───────────────────────────────────────
        let food: u32 = loop {
            match ui.ask("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE? ")? {
                PlayerInput::Quit => return ui.quit_game(),
                PlayerInput::Amount(n) => match state.allocate_food(n) {
                    Ok(()) => break n,
                    Err(NotEnough::Grain) => ui.not_enough_grain(state.grain)?,
                    _ => unreachable!(),
                },
            }
        };

        // ── Plant seed (lines 440-510) ────────────────────────────────────────
        let planted: u32 = loop {
            match ui.ask("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED? ")? {
                PlayerInput::Quit => return ui.quit_game(),
                PlayerInput::Amount(n) => match state.plant_seed(n) {
                    Ok(()) => break n,
                    Err(NotEnough::Acres) => ui.not_enough_acres(state.acres)?,
                    Err(NotEnough::Grain) => ui.not_enough_grain(state.grain)?,
                    Err(NotEnough::Workers) => ui.not_enough_workers(state.population)?,
                },
            }
        };

        match state.end_year(food, planted) {
            YearOutcome::Continue => {}
            YearOutcome::Impeached => {
                ui.impeach(state.starved, true)?;
                ui.farewell()?;
                return Ok(());
            }
        }
    }

    // ── End-of-term evaluation (lines 860-975) ─────────────────────────────
    let acres_per_person = state.acres / state.population;
    let verdict = if state.avg_starvation_rate > 33.0 || acres_per_person < 7 {
        Verdict::Worst
    } else if state.avg_starvation_rate > 10.0 || acres_per_person < 9 {
        Verdict::Bad
    } else if state.avg_starvation_rate > 3.0 || acres_per_person < 10 {
        let assassins = state.assassins_roll();
        Verdict::Mediocre(assassins)
    } else {
        Verdict::Best
    };
    ui.end_of_term_eval(&state, acres_per_person, verdict)?;
    ui.farewell()
}
