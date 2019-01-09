extern crate rand;

use rand::Rng;

const ADJECTIVES: &str = include_str!("../data/adjectives.txt");
const ADJECTIVES_COUNT: usize = 18185;

const NOUNS: &str = include_str!("../data/nouns.txt");
const NOUNS_COUNT: usize = 82192;

pub fn random() -> String {
    let adjective_line: usize = rand::thread_rng().gen_range(0, ADJECTIVES_COUNT);
    let noun_line: usize = rand::thread_rng().gen_range(0, NOUNS_COUNT);

    let adjective = get_line_from(ADJECTIVES, adjective_line)
        .to_lowercase()
        .replace("_", "-")
        .replace("/", "-");

    let noun = get_line_from(NOUNS, noun_line)
        .to_lowercase()
        .replace("_", "-")
        .replace("/", "-");

    format!("{}-{}", adjective, noun)
}

fn get_line_from(content: &str, line_num: usize) -> String {
    content.lines().nth(line_num).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", super::random());
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
