// A data structure for describing a car
struct Car {
    make : String,
    model : String,
    top_speed: usize
}

fn main() {

    // One way to instantiate an instance of the Car struct is
    // to define the struct and it's fields like so.
    let daily_driver: Car = Car {
        make : String::from("Chevrolet"),
        model : String::from("Camaro"),
        top_speed : 180,
    };

    // To access the fields of a struct use the dot notation like so.
    println!("My daily driver is a {} {} with a top speed of {}.",
             daily_driver.make,
             daily_driver.model,
             daily_driver.top_speed);

    // Another way to instantiate an instance of the Car struct is to use
    // a function that returns a new instance like so.
    let show_car: Car = car_constructor(String::from("Chevrolet"),
                                       String::from("Corvette"), 
                                       180);

    println!("My show car is a {} {} with a top speed of {}.",
             show_car.make,
             show_car.model,
             show_car.top_speed);

}

// This function is used to create a new instance of the struct Car.
fn car_constructor(make : String, model : String, top_speed : usize) -> Car {
    Car {
        make,
        model,
        top_speed
    }
}
