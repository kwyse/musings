use clap::clap_app;

mod weight;

fn main() {
    let app = clap_app!(app =>
        (version: "1.0")
        (@subcommand weight =>
            (about: "Manipulate your personal weight log")
            (@subcommand status =>
                (about: "Show latest recorded weight and trend")
                (@arg SOURCE: -s --source +takes_value "Reads a CSV file for weight data")
            )
        )
    ).get_matches();

   if let Some(weight_app) = app.subcommand_matches("weight") {
        if let Some(status_app) = weight_app.subcommand_matches("status") {
            if let Some(source) = status_app.value_of("SOURCE") {
                use std::cmp::Ordering;
                use std::fs::File;
                use std::io::{self, Write};

                let csv = File::open(source);
                let log = weight::WeightLog::from_csv(csv.unwrap()).unwrap();

                let mut stream = io::stdout();
                let last = log.as_slice().last().unwrap();
                write!(stream, "Latest weight recorded: {}kg\t\t({})\n", last.weight(), last.timestamp());

                let moving_average_period = 14;
                let moving_average = log.moving_average(moving_average_period);
                if let Some(last) = moving_average.last() {
                    let len = moving_average.len();
                    if let Some(penultimate) = moving_average.get(len - 2) {
                        let trend = match last.cmp(penultimate) {
                            Ordering::Less => "down",
                            Ordering::Equal => "flat",
                            Ordering::Greater => "up",
                        };

                        write!(stream, "Trending weight: {}kg\t\t\t(trending {})\n", last.weight(), trend);
                    }
                }
            }
        }
   }
}
