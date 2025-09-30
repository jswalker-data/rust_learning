use std::io;

fn main() {
    
    let mut temp = String::new();

    println!("Farenheight to Celsius converter!");
    println!("Please enter the temperature in farenheight:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number!");
            return;
        }
    };

    println!("Farenheight: {temp}");
    
    let celc: f64 = (temp - 32.0) * (5.0/9.0);

    println!("Celcius: {:.2}", celc);

}
