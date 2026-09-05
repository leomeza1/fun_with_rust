// A data structure for describing a car
// Note that structs use curly brackets '{' and '}'
struct Car {
    make : String,
    model : String,
    top_speed: usize
}

// A tuple structure for describing a point in 3D space
// Note that tuple structs use parentheses '(' and ')'
struct Point3D (i32, i32, i32);

// A tuple structure for describing a color
// Note that tuple structs use parentheses '(' and ')'
struct ColorRGB (i32, i32, i32);

// A unit-like struct.
struct UnitLike;

// A data structure for modeling a rectangle
struct Rectangle {
    width : u32,
    height : u32,
}

// A data structure for modeling a square
struct Square {
    width : u32,
}

// And implementation block for defining behaviors to Rectangles
impl Rectangle {

    // Returns the area of a rectangle. Note that the self
    // parameter gives the function an immutable reference,
    // so the function is not allowed to change any state of
    // the referenced object (cannot change width or height)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Takes a mutable reference to a rectangle and increases
    // the width by 1.
    fn grow_width(&mut self) {
        self.width += 1
    }

    // Takes ownership of a Rectangle and returns a Square. The
    // data owned by the Rectangle is moved. Thus, when this
    // function returns, the Rectangle is no longer valid.
    fn make_square(self) -> Square {
        Square { 
            width : self.height  // could have used self.width
        }
    }

    // An associated function like this one does not take 'self'
    // as a parameter and does not get invoked on a particular
    // object. Instead, an associated function is invoked on the
    // type itself using the '::' notation like 'Rectangle::new'.
    // This particular associated function returns a new instance
    // of a Rectangle.
    fn new(width : u32, height : u32) -> Self {
        Self {
            width,
            height
        }
    }
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

    // Note the use of parenthases '(' and ')'
    let black: ColorRGB = ColorRGB(0, 0, 0);
    let origin : Point3D = Point3D(0, 0, 0);

    // pass a ColorRGB object to a function that expects a ColorRGB object
    print_color(black);

    // tuple struct data can be accessed using pattern matching (destructuring)
    let Point3D(x, y, z) = origin;

    println!("The origin is at {}, {}, {}.", x, y, z);

    //-----------------------------------------------------------------
    // This function call will fail to compile because the function is
    // expecting a ColorRGB, but we're giving it a Point3D. This type
    // safety is an improvement over basic tuples.
    //-----------------------------------------------------------------
    //print_color(origin);


    //-----------------------------------------------------------------
    //
    //                      Unit-Like Structs
    //
    //-----------------------------------------------------------------

    // A unit-like struct is useful when we need to add behaviors to a
    // type using traits. This example will be expanded later to include
    // demonstration of how to implement a trait.
    let _ul_struct: UnitLike = UnitLike;


    //-----------------------------------------------------------------
    //
    //                   behavior with impl blocks
    //
    //-----------------------------------------------------------------

    // Note that the rectangle is mutable because we use the grow_width
    // function which changes state data in the rectangle
    let mut rectangle : Rectangle = Rectangle { width: 10, height: 10 };

    println!("The area of the rectangle is {}", rectangle.area());

    // Call grow_width to increase the width of the rectangle
    rectangle.grow_width();

    println!("Now the area of the rectangle is {}", rectangle.area());

    // Create a Square using the 'rectangle.make_square' function. Note
    // that this function takes ownership of the rectangle data and moves
    // the data to create the Square. Thus, the rectangle variable no
    // longer owns any data, it would be invalid to use rectangle after
    // this function returns.
    let square : Square = rectangle.make_square();

    println!("The area of the square is {}", square.width * square.width);

    // This will not compile because the rectangle was moved when we
    // called the 'make_square' function. So, the rectangle variable no
    // longer has ownership of the data (width and height) that it had
    // before.
    //println!("Now the area of the rectangle is {}", rectangle.area());


    //-----------------------------------------------------------------
    //
    //                     Associated functions
    //
    //-----------------------------------------------------------------

    // This example shows how to invoke a function associated with the
    // Rectangle type to create a new object. The function is named 'new',
    // but it could be named anything. The name 'new' is just something
    // that tends to be used for constructors like this.
    let another_rectangle : Rectangle = Rectangle::new(25, 50);

    println!("Now the area of the rectangle is {}", another_rectangle.area());

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

fn print_color(color : ColorRGB) {

    println!("The RGB code for this color is {}, {}, {}.",
             color.0, color.1, color.2);
}
