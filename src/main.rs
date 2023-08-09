use std::io;

fn main() {
    celsius_to_fahrenheit()
}

fn celsius_to_fahrenheit() {
    loop {
        println!("Please enter temperature in celsius.");
        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: i32 = celsius.trim().parse().expect("Celsius failed to pass!");
        let fahrenheit = (celsius * 9 / 5) + 32;

        println! {"The value of {celsius} degree clesius is: {fahrenheit}F"};
    }
}
