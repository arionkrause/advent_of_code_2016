pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let mut decompressed_string = String::new();
        let mut iterator = input.chars();

        'main: while let Some(character) = iterator.next() {
            if character.is_ascii_alphabetic() {
                decompressed_string.push(character);
            } else {
                let mut amount_characters_to_repeat = String::new();

                while let Some(character) = iterator.next() {
                    if character.is_digit(10) {
                        amount_characters_to_repeat.push(character);
                    } else {
                        let mut repetitions = String::new();

                        while let Some(character) = iterator.next() {
                            if character.is_digit(10) {
                                repetitions.push(character);
                            } else {
                                let mut characters_to_repeat = String::new();

                                for _ in 0..amount_characters_to_repeat.parse().unwrap() {
                                    characters_to_repeat.push(iterator.next().unwrap());
                                }

                                decompressed_string.push_str(&characters_to_repeat.repeat(repetitions.parse().unwrap()));
                                continue 'main;
                            }
                        }
                    }
                }
            }
        }

        decompressed_string.len()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("ADVENT"), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("A(1x5)BC"), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("(3x3)XYZ"), 9);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("A(2x2)BCD(2x2)EFG"), 11);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("(6x1)(1x3)A"), 6);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve("X(8x2)(3x3)ABCY"), 18);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> usize {
        get_decompressed_length(&input)
    }

    fn get_decompressed_length(input: &str) -> usize {
        let mut length = 0;
        let mut iterator = input.chars();

        'main: while let Some(character) = iterator.next() {
            if character.is_ascii_alphabetic() {
                length += 1;
            } else {
                let mut amount_characters_to_repeat = String::new();

                while let Some(character) = iterator.next() {
                    if character.is_digit(10) {
                        amount_characters_to_repeat.push(character);
                    } else {
                        let mut repetitions = String::new();

                        while let Some(character) = iterator.next() {
                            if character.is_digit(10) {
                                repetitions.push(character);
                            } else {
                                let mut characters_to_repeat = String::new();

                                for _ in 0..amount_characters_to_repeat.parse().unwrap() {
                                    characters_to_repeat.push(iterator.next().unwrap());
                                }

                                length += get_decompressed_length(&characters_to_repeat.repeat(repetitions.parse().unwrap()));
                                continue 'main;
                            }
                        }
                    }
                }
            }
        }

        length
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("(3x3)XYZ"), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("X(8x2)(3x3)ABCY"), 20);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}