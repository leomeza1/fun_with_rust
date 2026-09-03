// A data structure for describing a car
struct Car {
    make : String,
    model : String,
    top_speed: usize
}

// A tuple structure for describing a point in 3D space
struct Point (i32, i32, i32);

// A tuple structure for describing a color
struct Color (i32, i32, i32);

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

    // Struct Update Syntax (copy only, no moves):
    //-------------------------------------------------
    // This is a way to instantiate another Car instance using
    // some of the data from an existing Car instance. In this
    // example the 'previous_car' object is created using the
    // specified make and model fields, but then we tell the
    // Car constructor to copy the rest of the fields from the
    // 'show_car' object. Note that the only field copied from
    // the 'show_car' object is the 'top_speed' field which is
    // of type usize. The usize type implements the Copy trait.
    // So, all is good. Both 'show_car' and 'previous_car' are
    // valid for use anywhere we'd normally use them. Problems
    // can occur when we tell the constructor to use data from
    // fields of another object which don't implement the Copy
    // trait. For example, the make and model fields are both
    // of type String which does not implement the Copy trait.
    // Using those fields in the constructor would cause a
    // move from the 'show_car' to the 'previous_car'.
    let previous_car : Car = Car {
        make : String::from("Ford"),
        model : String::from("Mustang"),
        ..show_car
    };

    println!("My previous car was a {} {} with a top speed of {}.",
             previous_car.make,
             previous_car.model,
             previous_car.top_speed);

    // Struct Update Syntax (with move):
    //-------------------------------------------------
    // This example presents a possible problem because we tell the
    // constructor to use 'previous_car.make' field when creating
    // the 'next_car.make' field. However, the 'previous_car.make'
    // field is a String which doesn't implement the Copy trait.
    // So, instead of copying from 'previous_car.make' we actually
    // move ownership of it. If we try to use the 'previous_car.make'
    // field later on, the borrow checker will complain. Remember
    // the second rule of ownership, each value can have only one
    // owner at a time.
    let next_car : Car = Car {
        model : String::from("F-150"),
        top_speed : 120,
        ..previous_car  // move from 'previous_car.make'
    };

    println!("My next car will be a {} {} with a top speed of {}.",
             next_car.make,
             next_car.model,
             next_car.top_speed);

    //-----------------------------------------------------------------
    // This println will fail to compile because ownership of the value
    // in 'previous_car.make' was moved to 'next_car.make'.
    // Thanks borrow checker!
    //-----------------------------------------------------------------
    //println!("My previous car was a {} {} with a top speed of {}.",
    //         previous_car.make,
    //         previous_car.model,
    //         previous_car.top_speed);


    //-----------------------------------------------------------------
    //
    //                         Tuple Structs
    //
    //-----------------------------------------------------------------

    let black: Color = Color(0, 0, 0);
    let origin : Point = Point(0, 0, 0);

    // pass a Color object to a function that expects a Color object
    print_color(black);

    // tuple struct data can be accessed using pattern matching (destructuring)
    let Point(x, y, z) = origin;

    println!("The origin is at {}, {}, {}.", x, y, z);

    //-----------------------------------------------------------------
    // This function call will fail to compile because the function is
    // expecting a Color, but we're giving it a Point. This type safety
    // is an improvement over basic tuples.
    //-----------------------------------------------------------------
    //print_color(origin);

}

// This function is used to create a new instance of the struct Car.
fn car_constructor(make : String, model : String, top_speed : usize) -> Car {

    // Field Init Shorthand:
    //------------------------
    // Since the field names (make, model, and top_speed) of the Car
    // struct are the same as the function parameter names of the
    // 'car_constructor' function we can simply pass the names into
    // the Car constructor when instantiating the object instance.
    Car {
        make,
        model,
        top_speed
    }
}

fn print_color(color : Color) {

    println!("The RGB code for this color is {}, {}, {}.",
             color.0, color.1, color.2);
}
