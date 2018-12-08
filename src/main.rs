use std::io;

fn main() {
    println!("=========================================");
    println!("| Welcome to the temperature converter! |");
    println!("=========================================\n");

    println!("To begin, enter 'F' for Fahrenheit or 'C' for Celsius: ");
    println!(".......................................................");


    loop {
        let mut temp_type = String::new();

        io::stdin().read_line(&mut temp_type)
            .expect("Failed to read line");

        if temp_type.trim().to_lowercase() == "f" {
            get_temp(temp_type.trim().to_lowercase());
            break;

        } else if temp_type.trim().to_lowercase() == "c" {
            get_temp(temp_type.trim().to_lowercase());
            break;
        } else {
            println!("Please choose F or C");
        }
    }
}

fn get_temp(temp_type: String){
    loop {
        let mut temp = String::new();    
        
        println!("Please enter the temperature you wish to convert from: ");

        io::stdin().read_line(&mut temp)
            .expect("Failed to get temperature");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temp_type == "f" {
            f_to_c(temp);
        } else if temp_type == "c" {
            c_to_f(temp);
        }
        break;
    }
}

fn f_to_c(temp: i32) {
    //    (32°F − 32) × 5/9 = 0°C
    let celsius = (temp - 32) * 5 / 9;
    println!("{}F is equal to {}C", temp, celsius);
    convert_again();
}
    
fn c_to_f(temp: i32) {
    // (32°C × 9/5) + 32 = 89.6°F
    let fahrenheit = (temp * 9 / 5) + 32;
    println!("{}C is equal to {}F.", temp, fahrenheit);
    convert_again();
}

fn convert_again() {
    let mut again = String::new();

    println!("Would you like to convert another temperature? Press Y to continue | N to exit.");
    loop {

        io::stdin().read_line(&mut again)
            .expect("Failed to get input");

        if again.trim().to_lowercase() == "n" {
            println!("Thanks for using Convert Temp.");
            break;
        } else if again.trim().to_lowercase() == "y" {
            main();
        }
        else {
            println!("Please choose Y or N");
        }
        break;
    }
}