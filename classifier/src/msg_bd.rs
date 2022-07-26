// declare a red black tree 
// declare a message board class
// declare a message class
// declare a message board classifier class
// declare a message board classifier classifier class


struct msg_bd {
    num_posts: i32,
    vocab_size, i32,
    posts_containing_word: HashMap<String, int>,
    posts_containing_label: HashMap<String, int>,
    things: Vec<String>,
    preds: Vec<String>,
    content: Vec<String>,
    max_probb: Vec<f64>,
    posts_with_label_containing_word: HashMap<<string,string>, int>,
}

impl msg_bd {
    // function returns a hash set
    pub fn find_unique_words(const string& s) -> HashSet<string> {
        let mut word = String::new();
        // hashshet of strings
        let mut unique_words = HashSet::new();
        // read in from a file
        let mut file = File::open(s).unwrap();
        // read in the file
        let mut reader = BufReader::new(file);
        // read in the file line by line
        for line in reader.lines() {
            // split the line into words
            let words = line.unwrap().split_whitespace();
            // iterate through the words
            for word in words {
                // add the word to the hashset
                unique_words.insert(word);
            }
        }
        // return the hashset
        unique_words
    }
    // void function that reads in the file and outputs things
    pub fn read_in_file(const string& s) {
        let mut row = HashMap<string, string>::new();
        println!("Training data:");
        // read in from a file
        let mut file = File::open(s).unwrap();
        let mut reader = BufReader::new(file);
        while let Some(line) = reader.lines().next() {
            let mut thing = row["tag"];
            let mut unique_words = find_unique_words(row["content"]);
            posts_containing_label[thing] += 1;
            num_posts += 1;
            faster_words(unique_words, thing);
            vocab_size = unique_words.len();
        }
        println!("Trained on {} examples", num_posts);
    }

    pub fn read_in_file_thorough(const string& s) {
        let mut row = HashMap<string, string>::new();
        println!("Training data:");
        // read in from a file
        let mut file = File::open(s).unwrap();
        let mut reader = BufReader::new(file);
        while let Some(line) = reader.lines().next() {
            let mut thing = row["tag"];
            let mut unique_words = find_unique_words(row["content"]);
            posts_containing_label[thing] += 1;
            println!(" label = {1}, content = {2}", thing, row["content"]);
            num_posts += 1;
            faster_words(unique_words, thing);
            vocab_size = unique_words.len();
        }
        println!("Trained on {} examples", num_posts);
        println!("Vocabulary size = {}", vocab_size);
        println!("classes");
        // iterate through "posts_containing_label"
        for (key, value) in posts_containing_label.iter() {
            println!(" {1}, {2}, examples, log-prior = {3}", key, value, (posts_containing_label[key] as f64) / (num_posts as f64).log2());
        }
        println!("classifier parameters");
        // iterate through "posts_with_label_containing_word"
        for (key, value) in posts_with_label_containing_word.iter() {
            println!(" {1}, {2}, examples, count = {3}", key.0, key.1, value, (value as f64) / (posts_containing_label[key.0] as f64).log2());
        }
    }

    pub fn faster_words(const HashSet<string>& unique_words, const string& row) {
        // iterate through hash set
        for word in unique_words {
            let mut pww = (row, word);
            posts_with_label_containing_word[pww] += 1;
            posts_containing_word[word] += 1;
        }
    }

    pub fn testing_data(const string& s) {
        let mut row = HashMap<string, string>::new();
        let mut num_correct = 0;
        let mut num_total = 0;
        println!("Test data:");
        let mut file = File::open(s).unwrap();
        let mut reader = BufReader::new(file);
        while let Some(line) = reader.lines().next() {
            let mut thing = row["tag"];
            let mut unique_words = find_unique_words(row["content"]);
            let mut pred = "";
            let mut max_prob = -1e10;
            // iterate through "posts containing label"
            for (key, value) in posts_containing_label.iter() {
                let label = key;
                let mut log_prob_temp = log_prob_calc(label, unique_words) as f64;
                if log_prob_temp > max_prob {
                    max_prob = log_prob_temp;
                    pred = label;
                }
            }
            things.push(thing);
            preds.push(pred);
            content.push(row["content"]);
            max_probb.push(max_prob);
            if pred == thing {
                num_correct += 1;
            }
            num_total += 1;

        }
        for i in 0..things.len() {
            println!(" correct = {1}, predicted = {2}, log-probability score = {3},
                content = {4}", things[i], preds[i], max_probb[i], content[i]);
            println!("performance: {1}/{2} = {3} posts predicted correctly", num_correct, num_total, (num_correct as f64) / (num_total as f64));
        }
    }

    pub fn log_prob_calc(const string& label, HashSet& words) -> f64 {
        let mut log_prob = ((posts_containing_label[label] as f64) / (num_posts as f64)).log2();
        for word in words {
            let mut pww = (label, word);
            if posts_with_label_containing_word[pww] > 0) {
                log_prob += (posts_with_label_containing_word[pww] as f64 / posts_containing_label as f64).log2();
            }
            else if posts_containing_word[word] > 0 {
                log_prob += (posts_containing_word[word] as f64 / num_posts).log2();
            }
            else {
                log_prob += (1.0 / num_posts).log2();
            }
        }
        log_prob
    }
}