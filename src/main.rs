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
                use std::fs::File;
                use std::io::{self, Write};

                let csv = File::open(source);
                let log = weight::WeightLog::from_csv(csv.unwrap()).unwrap();

                let mut stream = io::stdout();
                let last = log.as_slice().last().unwrap();
                write!(stream, "Latest weight recorded: {}kg\t\t({})\n", last.weight(), last.timestamp());
            }
        }
   }
}
