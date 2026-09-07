mod kyu8;

fn main() {
    println!("Hello, world!");
    println!("{}", kyu8::tasks::get_grade(82, 85, 87));
    println!();
    println!("=============================================");
    println!();
    println!(
        "{}",
        kyu8::tasks::no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string())
    );
    println!("=============================================");
    println!();
    println!("{}", kyu8::tasks::multi_table(3));
}
