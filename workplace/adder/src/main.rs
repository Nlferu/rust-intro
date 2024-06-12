use add_one;

fn main() {
    let num = 776;

    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_rand(num)
    );
}
