mod f;
mod hello_user;
mod tuple;
fn main() {
    println!("Hello, world!");
    f::write_f();
    tuple::tuple();
    hello_user::hello("Siddhesh");
    let arr = [0; 16];
    println!("{}", arr[0]);
}
