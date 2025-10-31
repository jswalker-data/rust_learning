
// enum says the value is one of a possible set of values
// encode possibilities e.g. shapes enum has circle, square etc.

// e.g. IP adresses only come in version 4 or 6
// we can ENUMERATE all possibilities
enum IpAddrKind {
    V4,
    V6,
}

// create instances like this
// both of these are of the same data type: IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// we can define a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {}

// and call with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);

// to assign data to an enum we could use a struct:

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

// same concept with just an enum
// attach data directly to the enum
// the name of each enum variant also becomes a function that
// constructs an instance of the enum
// IpAddr::V4() is a fn that takes a String and returns instance of IpAddr type
enum IpAddr {
    V4(String),
    V6(String,)
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// each variant can have different types and amounts associated with it




