use log::{ info, debug };
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod calorie_counting;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).with_utc_timestamps().init().unwrap();

    info!("Santa's coming to town.");
    debug!("Santa's coming to town.");
    day01("data/01-input");

    /// Day 01
    fn day01(datafile: &str) {
        info!("==== DAY 01 - PART ONE ====");

        let contents = std::fs
            ::read_to_string(datafile)
            .expect("Something went wrong reading the file");

        let biggest_elf = calorie_counting::find_biggest_elf(&contents);
        let biggest_three = calorie_counting::find_big_three_elf(&contents);
        info!("==== DAY 01 - SOLUTION 1 ====");
        info!("======= {:?} ========", biggest_elf);
        info!("==== DAY 01 - SOLUTION 2 ====");
        info!("======= {:?} ========", biggest_three);
        info!("====== DAY 01 - END =======");
    }
}