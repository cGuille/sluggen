extern crate rand;
extern crate regex;

use rand::Rng;
use regex::Regex;

const ADJECTIVES: &str = include_str!("../data/adjectives.txt");
const ADJECTIVES_COUNT: usize = 18185;

const NOUNS: &str = include_str!("../data/nouns.txt");
const NOUNS_COUNT: usize = 82192;

pub fn random() -> String {
    let adjective_line: usize = rand::thread_rng().gen_range(0, ADJECTIVES_COUNT);
    let noun_line: usize = rand::thread_rng().gen_range(0, NOUNS_COUNT);

    let adjective = get_line_from(ADJECTIVES, adjective_line);
    let noun = get_line_from(NOUNS, noun_line);

    format!("{}-{}", slugify(adjective), slugify(noun))
}

fn slugify(value: &str) -> String {
    let non_slug_chars = Regex::new(r"[^A-Za-z]+").unwrap();
    non_slug_chars.replace_all(value, "-").to_lowercase()
}

fn get_line_from(content: &str, line_num: usize) -> &str {
    content.lines().nth(line_num).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_slug_generation() {
        println!("{}", super::random());
    }

    #[test]
    fn slugify() {
        assert_eq!(super::slugify("OHAI-00_ :) that works"), "ohai-that-works");
    }

    #[test]
    fn get_line_from() {
        let content = "First line
Second line";

        assert_eq!(super::get_line_from(content, 0), "First line");
        assert_eq!(super::get_line_from(content, 1), "Second line");
    }

    #[test]
    #[should_panic]
    fn get_line_from_panics() {
        let content = "First line
Second line";

        // valid indices are 0 and 1, so 2 should make the fn panic:
        super::get_line_from(content, 2);
    }
}
