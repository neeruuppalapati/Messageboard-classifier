use msg_bd::*;
use std::collections::HashMap;
use std::collections::HashSet;

// main reads in command line arguments
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut msg_bd = msg_bd {
        num_posts: 0,
        vocab_size: 0,
        posts_containing_word: HashMap::new(),
        posts_containing_label: HashMap::new(),
        things: Vec::new(),
        preds: Vec::new(),
        content: Vec::new(),
        max_probb: Vec::new(),
        posts_with_label_containing_word: HashMap::new(),
    };
    // open argument 1
    let mut file = File::open(&args[1]).unwrap();
    // open argument 2
    let mut file2 = File::open(&args[2]).unwrap();
    // if argument 3 is true read in thorough
    if args[3] == "true" {
        msg_bd.read_in_file_thorough(&args[1]);
    } else {
        msg_bd.read_in_file(&args[1]);
    }
    msg_bd.testing_data(&args[2]);
}
