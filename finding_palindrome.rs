fn main() {
    let words = ["he", "names", "malayalam"];
    words.iter().for_each(|names| {
        let reversed_names : String = names.chars().rev().collect();
        if names != &reversed_names {
            println!("{}", names);
        }
    });
}
