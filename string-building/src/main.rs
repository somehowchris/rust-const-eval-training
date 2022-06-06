use const_format::{concatcp, str_get};

const NAME: &str = "Christof";

struct MeType {
    name: &'static str,
}

const fn get_me() -> MeType {
    MeType {
        name: "Christof"
    }
}

const fn hello_me<>() -> &'static str {
    concatcp!("Hello ", get_me().name)
}

const ME_NAME: &'static str = {
    let me = get_me();
    me.name
};

const fn get_string_length(input: &str) -> usize {
    input.len()
}

const NAME_LENGTH: usize = get_string_length(NAME);

struct TreeElement<'a, A, B, Outcome> {
    pub handle: &'a dyn Fn(A, B) -> Outcome,
    pub path: &'static str,
}

fn test(_me: i32, hey: u8) -> u8 {
    2u8
}

fn test2(_me: i32, hey: i32) -> u128 {
    2u128
}

const fn get_duplicates(inputs: &[&str]) -> usize {
    let mut dupe_count = 0;

    let mut count = 0;

    dupe_count
}

fn main() {
    println!("str NAME: {}", NAME);
    println!("String NAME: {}!",ME_NAME);
    println!("get_me().name: {}!", get_me().name);
    println!("hello_me(): {}!", hello_me());
    println!("NAME_LENGTH: {}!", NAME_LENGTH);

    let element = TreeElement {
        handle: &test,
        path: "/"
    };

    let element2 = TreeElement {
        handle: &test2,
        path: "/test"
    };

    let handle = element.handle;

    println!("Value: {}", handle(4i32, 4u8));


    let handle2 = element2.handle;

    println!("Value: {}", handle2(4i32, 4i32));
}
