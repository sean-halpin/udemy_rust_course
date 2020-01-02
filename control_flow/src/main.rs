fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("Quite hot out there !");
    } else if temp < 10 {
        println!("Quite chilly out there !");
    } else {
        println!("Temperature is nice today.")
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Today is {}", day);

    println!("It is {}", if temp > 20 { "nice" } else { "gloomy" });

    println!(
        "It is {}",
        if temp > 20 {
            if temp > 30 {
                "Very Hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "Cold"
        } else {
            "Okay"
        }
    );
}

fn main() {
    if_statement();
}
