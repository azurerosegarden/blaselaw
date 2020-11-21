mod case;
mod modifiers;
mod player;
mod team;

fn main() {
    // Generate a new Case
    let the_case = case::Case::new();

    println!("{:#?}", the_case);
    println!("{}", the_case.get_case_name())
}
