fn canada_to_poland_hours(canada_hours: u8) -> u8 {
    (canada_hours + 6) % 24
}

fn main() {
    println!("CANADA --- POLAND\n");

    for hour in 0..24 {
        let poland_hours = canada_to_poland_hours(hour);

        println!(
            "{:0>2}:{:0>2} ----- {:0>2}:{:0>2}",
            hour, 0, poland_hours, 0
        );
    }
}
