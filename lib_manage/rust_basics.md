### let

let instroduces a variable binding
let x; - declare "x"
x = 42; - assign 42 to "x"

single line

let x = 42;

### types

type annotation
let x: i32;
x = 42;

single line
let x: i32 = 42

### i32 

Equivilent to a c++ long, works for most things unless specifically needing a certain size or size is pre-known. It is the default number size.

### _s

this does nothing
let _ = 42;

this calls the func but discards result
let _ = get_thing();

### tuples

let pain = ('a', 17);
pair.0; - this is 'a'
pair.1; - this is 17

explicit type annotation
let pair: (char, i32) = ('a', 17);

let (some_char, some_int) = ('a', 17);
assert!(some_char, 'a');
assert!(some_int, 17);

destructure tuple
let (l, r) = slice.split_at(middle);
let (_, right) = slice.split_at(middle);

### ;s

semicolons signal the end of a line

;s can span multiple lines

let x = vec![1,2,3,4,5,6,7,8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);

### fn

fn = functions

f -> void
fn greet(){
    println!("Hi there");
}

f -> i32 (arrow = return type)
fn fair_dice_roll() -> i32 {
    4
}

when referencing for fn call (print_name(&user2)) the "&" makes the variable immutable

### structs

init a new struct for user
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand to pull from funct var
        username, // shorthand to pull from funct var
        active: true,
        sign_in_count: 1,
    }
}

fn main(){
    let user1 = User {
        email: String::from("Foo"),
        username: String::from("Bar"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("NameHere");

    let uyser2 = buld_user(
        String::from("emailHere"),
        String::from("unameHere")
    );
}