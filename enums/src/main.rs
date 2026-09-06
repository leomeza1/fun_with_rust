
// A simple enumeration that defines two values. A variable of
// this kind can either be an IpAddrKind::V4 or an IpAddrKind::V6,
// but not both.
enum IpAddrKind {
    V4,
    V6,
}

// A slightly more sophisticated example of an enumberation.
// This one associates data with the enumeration. A variable of
// this kind can either be an IpAddrKind::V4 or an IpAddrKind::V6,
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
// of the tuples used by the 'IpAddr' enum example above.
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

// Enumeration types can have impl blocks just like structs do.
impl Message {

    fn call(&self) {
        // TBD
    }
}

// Functions can take enumeration types.
fn route_ip_addr_kind(_ip_addr_kind : IpAddrKind) {

}

fn route_ip_addr(_ip_addr : IpAddr) {

}

fn route_ip_address(_ip_address : IpAddress) {

}

fn main() {

    // Instantiation of simple enumeration types
    let ip_v4 : IpAddrKind  = IpAddrKind::V4;
    let ip_v6 : IpAddrKind = IpAddrKind::V6;

    // Enumerations can be passed as function parameters
    route_ip_addr_kind(ip_v4);
    route_ip_addr_kind(ip_v6);
    route_ip_addr_kind(IpAddrKind::V4);

    // Instantiation of the enumeration types that associate
    // data with the enumeration using tuples.
    let ip_addr_v4 : IpAddr = IpAddr::V4(127, 0, 0, 1);
    let ip_addr_v6 : IpAddr = IpAddr::V6(String::from("::01"));

    route_ip_addr(ip_addr_v4);
    route_ip_addr(ip_addr_v6);

    // Instantiation of the enumeration types that associate
    // data with the enumeration usin structs.
    let ip_address_v4 : IpAddress = IpAddress::V4( IpV4Addr{first : 127, second : 0, third : 0, fourth : 1});
    let ip_address_v6 : IpAddress = IpAddress::V6( IpV6Addr{address : String::from("::1")});

    route_ip_address(ip_address_v4);
    route_ip_address(ip_address_v6);

    // Instantiation of the Message enumeration type, this one
    // is associated with string data.
    let msg : Message = Message::Write(String::from("Hello World!"));

    msg.call();

}
