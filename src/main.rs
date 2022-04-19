use colored::Colorize;

fn main() {
    println!("CANADA ---- POLAND\n");

    for canada_time in 0..24 {
        if canada_time >= 8 && canada_time <= 23 {
            print!("{}", format!("{:0>2}:00 --", canada_time).green());
        } else {
            print!("{}", format!("{:0>2}:00 --", canada_time).red());
        }

        let poland_time = (canada_time + 6) % 24;
        if poland_time >= 8 && poland_time <= 23 {
            println!("{}", format!("-- {:0>2}:00", poland_time).green());
        } else {
            println!("{}", format!("-- {:0>2}:00", poland_time).red());
        }
    }
}
