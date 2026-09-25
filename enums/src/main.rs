
// A simple enumeration that defines two values. A variable of
// this type can either be an IpAddrKind::V4 or an IpAddrKind::V6,
// but not both.
enum IpAddrKind {
    V4,
    V6,
}

// A slightly more sophisticated example of an enumberation.
// This one associates data with the enumeration. A variable of
// this type can either be an IpAddrKind::V4 or an IpAddrKind::V6,
// but not both. In addition, if the variable is an IpAddrKind::V4,
// then it also owns a tuple of four u8 data types. If the variable
// is an IpAddrKind::V6, then it also owns a string.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another way to represent the IPv4 data is with a struct.
// This struct is used in the 'IpAddress' enum below. This is
// an another way of defining the data if you want to name the
// different pieces of data. Note that the example above uses
// a tuple which doesn't name the four u8 data types.
struct IpV4Addr {
    first  : u8,
    second : u8,
    third  : u8,
    fourth : u8
}

// Another way to represent the IPv6 data using a struct with
// named fields.
struct IpV6Addr {
    address : String
}

// Another example of how to associate data with an enumeration.
// This example uses the structs 'IpV4Addr' and 'IpV6Addr' instead
// of the tuples and String used by the 'IpAddr' enum example above.
enum IpAddress {
    V4(IpV4Addr),
    V6(IpV6Addr), 
}

// Yet another example of how to associate data with an enumeration.
// This time, we're defining an enumeration that is a bit more
// generic. This enumeration is a Message that can be any one of
// several different things. The simplist Message being the 'Quit'
// message which does not have any data associated with it. The
// other messages have data associated with them.
enum Message {
    Quit,
    Move { x : i32, y : i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enumeration types can have impl blocks that define behavior
// just like how structs can.
impl Message {

    fn call(&self) {
        // TBD
    }
}

// This functions takes an IpAddrKind enumeration type.
fn route_ip_addr_kind(_ip_addr_kind : IpAddrKind) {

}

// This function takes an IpAddr enumeration type.
fn route_ip_addr(_ip_addr : IpAddr) {

}

// This function takes an IpAddress enumeration type.
fn route_ip_address(_ip_address : IpAddress) {

}

// This function takes an Option<i32> and does one of the following:
// 1) If None then returns None
// 2) If Some(value) then returns Some(value+1)
fn option_i32_plus_one(x : Option<i32>) -> Option<i32> {

    match x {

        // Obviously, just return 'None' if the input parameter 'x'
        // does not contain a value. This is similar to null in C/C++.
        None => None,

        // If the input parameter 'x' contains a value, bind our local
        // variable 'integer' to it and return an Option<i32> that has a
        // value equal to 'integer' + 1.
        Some(integer) => Some(integer + 1),
    }
}

fn main() {

    // Instantiation of simple enumeration types
    let ip_v4 : IpAddrKind  = IpAddrKind::V4;
    let ip_v6 : IpAddrKind = IpAddrKind::V6;

    // Enumerations can be passed as function parameters
    route_ip_addr_kind(ip_v4);
    route_ip_addr_kind(ip_v6);
    route_ip_addr_kind(IpAddrKind::V4);
    route_ip_addr_kind(IpAddrKind::V6);

    // Instantiation of the enumeration types that associate
    // data with the enumeration using tuples and Strings.
    let ip_addr_v4 : IpAddr = IpAddr::V4(127, 0, 0, 1);
    let ip_addr_v6 : IpAddr = IpAddr::V6(String::from("::01"));

    route_ip_addr(ip_addr_v4);
    route_ip_addr(ip_addr_v6);

    // Instantiation of the enumeration types that associate
    // data with the enumeration using structs.
    let ip_address_v4 : IpAddress = IpAddress::V4( IpV4Addr{first : 127, second : 0, third : 0, fourth : 1} );
    let ip_address_v6 : IpAddress = IpAddress::V6( IpV6Addr{address : String::from("::1")} );

    route_ip_address(ip_address_v4);
    route_ip_address(ip_address_v6);

    // Instantiation of the Message enumeration type, this one
    // is associated with string data.
    let msg : Message = Message::Write(String::from("Hello World!"));

    msg.call();

    //-----------------------------------------------------------------
    //
    //                    Pattern Matching
    //
    //-----------------------------------------------------------------

    let ip_addr_kind : IpAddrKind = IpAddrKind::V4;

    match ip_addr_kind {

        IpAddrKind::V4 => {
            println!("ip_addr_kind is an IpAddrKind::V4");
        }

        IpAddrKind::V6 => {
            println!("ip_addr_kind is an IpAddrKind::V6");
        }
    }

    //-----------------------------------------------------------------
    //
    //              Pattern Matching with Value Binding
    //
    //-----------------------------------------------------------------

    let ip_addr : IpAddr = IpAddr::V4(127, 0, 0, 1);

    match ip_addr {

        // Notice that the match pattern includes local variables named 'first',
        // 'second', 'third', and 'fourth'. These local variables bind to the values
        // in the ip_addr variable and then we can use those local variables within
        // the matching arm block.
        IpAddr::V4(first, second, third, fourth) => {
            println!("ip_addr is {:?}.{:?}.{:?}.{:?}", first, second, third, fourth);
        }

        // Same as above, but we bin the 'ip_address' local variable to the String
        // value in the IpAddr::V6 type.
        IpAddr::V6(ip_address) => {
            println!("ip_addr is {:?}", ip_address);
        }
    }

    //-----------------------------------------------------------------
    //
    //               Pattern Matching with Option
    //
    //-----------------------------------------------------------------

    let five: Option<i32> = Some(5);

    // go see the option_i32_plus_one function for a description
    let six: Option<i32> = option_i32_plus_one(five);
    let none: Option<i32> = option_i32_plus_one(None);

    println!("five is {}", five.expect("This error should never happen :)"));
    println!("six is {}", six.expect("This error should never happen :)"));
    println!("none is {}", none.unwrap_or_default());

    //-----------------------------------------------------------------
    //
    //               Pattern Matching with 'other'
    //
    //-----------------------------------------------------------------

    let balloons: i32 = 99;

    match balloons {

        99 => {
            println!("Nina would be proud of you!");
        },

        // By using 'other' here we are catching all other cases where
        // the value of 'balloons' is not equal to 99. This is similar
        // to the 'default' case in C/C++. However, we bind the value
        // of 'balloons' to 'other' so that we can use the value in the
        // arm block.
        other => {
            println!("You have {} balloons, congrats!", other);
        },
    }

    //-----------------------------------------------------------------
    //
    //               Pattern Matching with Underscore '_'
    //
    //-----------------------------------------------------------------

    let message: Message = Message::Write(String::from("Super awesome message!"));

    match message {

        Message::Write(str_msg) => {
            println!("message is: {:?}", str_msg);
        }

        // This arm will never get executed because message is a Message::Write type,
        // but by using the underscore, we satisfy the exhaustiveness requirement of
        // the pattern matching construct. This allows us to ignore the other cases
        // such as Quit, Move, and ChangeColor.
        _ => (),
    }

    //-----------------------------------------------------------------
    //
    //                    The 'if let' expression
    //
    //-----------------------------------------------------------------

    let cfg_val: Option<u8> = Some(3u8);

    // Another way to take action when a variable matches a single pattern
    // and ignore all other cases is to use the 'if let' expression. Here,
    // if the pattern matches, a block of code is executed. Otherwise, if
    // the pattern does not match, we can either do nothing or a add an
    // 'else' (or 'else if') expression.
    if let Some(config)= cfg_val {
        println!("config is: {:?}", config);
    }
    else {
        println!("config is not valid!");
    }

}
