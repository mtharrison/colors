use std::collections::HashMap;
use std::io::BufRead;

use crate::types::*;

pub fn parse<T: BufRead>(mut stream: T) -> Specification {
    // Parse number of colors to produce from stdin

    let mut input = String::new();

    stream
        .read_line(&mut input)
        .expect("Error reading from stdin");

    let num_colors = input
        .trim()
        .parse::<usize>()
        .expect("Error parsing num colors");

    // Loop through all customer preferences until done

    let mut preferences: Vec<CustomerPreferences> = vec![];

    loop {
        let mut input = String::new();

        let bytes_read = stream
            .read_line(&mut input)
            .expect("Error reading from stdin");

        if bytes_read == 0 {
            break;
        }

        let mut map: CustomerPreferences = HashMap::new();
        let mut buf = String::new();

        for c in input.chars() {
            if c.is_numeric() {
                buf.push(c);
            }

            if c.is_alphabetic() {
                let paint = buf.parse::<usize>().expect("Couldn't parse paint number");
                let finish = match c {
                    'M' => Finish::Matte,
                    'G' => Finish::Gloss,
                    _ => panic!("Unknown paint finish"),
                };
                buf.clear();
                map.insert(paint, finish);
            }
        }

        if !map.is_empty() {
            preferences.push(map);
        }
    }

    // Record any essential paint combinations
    // This is a tactic to reduce search space

    let mut essentials: CustomerPreferences = HashMap::new();

    for customer in &preferences {
        let count = customer.len();
        if count == 1 {
            let (key, value) = customer.iter().nth(0).unwrap();
            essentials.insert(*key, value.clone());
        }
    }

    Specification {
        num_colors,
        preferences,
        essentials,
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::io::BufReader;

    use crate::parser::parse;
    use crate::types::*;

    #[test]
    fn it_parses_input() {
        let input = "3\n1 M\n2 G 1 G\n3 M".as_bytes();
        let spec = parse(BufReader::new(input));

        let mut essentials = HashMap::new();
        essentials.insert(1, Finish::Matte);
        essentials.insert(3, Finish::Matte);

        let mut pref1 = HashMap::new();
        pref1.insert(1, Finish::Matte);

        let mut pref2 = HashMap::new();
        pref2.insert(2, Finish::Gloss);
        pref2.insert(1, Finish::Gloss);

        let mut pref3 = HashMap::new();
        pref3.insert(3, Finish::Matte);

        let preferences = vec![pref1, pref2, pref3];

        assert_eq!(
            spec,
            Specification {
                num_colors: 3,
                essentials,
                preferences
            }
        )
    }

    #[test]
    fn it_handles_extra_whitespace() {
        let input = "  5\n1   M\n2G\t1 G\n3 M  ".as_bytes();
        let spec = parse(BufReader::new(input));

        let mut essentials = HashMap::new();
        essentials.insert(1, Finish::Matte);
        essentials.insert(3, Finish::Matte);

        let mut pref1 = HashMap::new();
        pref1.insert(1, Finish::Matte);

        let mut pref2 = HashMap::new();
        pref2.insert(2, Finish::Gloss);
        pref2.insert(1, Finish::Gloss);

        let mut pref3 = HashMap::new();
        pref3.insert(3, Finish::Matte);

        let preferences = vec![pref1, pref2, pref3];

        assert_eq!(
            spec,
            Specification {
                num_colors: 5,
                essentials,
                preferences
            }
        )
    }

    #[test]
    fn it_handles_numbers_over_10() {
        let input = "15\n1 M\n2 G 1 G\n3 M 12M 9G".as_bytes();
        let spec = parse(BufReader::new(input));

        let mut essentials = HashMap::new();
        essentials.insert(1, Finish::Matte);

        let mut pref1 = HashMap::new();
        pref1.insert(1, Finish::Matte);

        let mut pref2 = HashMap::new();
        pref2.insert(2, Finish::Gloss);
        pref2.insert(1, Finish::Gloss);

        let mut pref3 = HashMap::new();
        pref3.insert(3, Finish::Matte);
        pref3.insert(12, Finish::Matte);
        pref3.insert(9, Finish::Gloss);

        let preferences = vec![pref1, pref2, pref3];

        assert_eq!(
            spec,
            Specification {
                num_colors: 15,
                essentials,
                preferences
            }
        )
    }
}
