use clap::{App, Arg};
use termion::color;

fn main() {
    // Define the command line arguments
    let matches = App::new("my_program")
        .arg(
            Arg::with_name("tree_height")
                .help("The height of the tree")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Parse the tree_height argument
    let tree_height: usize = matches
        .value_of("tree_height")
        .unwrap()
        .parse()
        .unwrap();

    // Print the top of the tree
    println!("{}{}{}", color::Fg(color::Yellow), " ".repeat(tree_height-1), "*");

    // Loop through the rows of the tree
    for i in 1..tree_height {
        // Select a random color for the asterisks
        let color_index = rand::random::<usize>() % 3;

        // Indent the rows of the tree
        let indent = " ".repeat(tree_height - i - 1);

        match color_index {
            0 => println!("{}{}{}", color::Fg(color::Green), indent, "*".repeat(2 * i + 1)),
            1 => println!("{}{}{}", color::Fg(color::Red), indent, "*".repeat(2 * i + 1)),
            2 => println!("{}{}{}", color::Fg(color::LightBlue), indent, "*".repeat(2 * i + 1)),
            _ => unreachable!(),
        };
    }
}
