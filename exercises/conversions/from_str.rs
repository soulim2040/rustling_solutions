// from_str.rs
// This is similar to from_into.rs, but this time we'll implement `FromStr`
// and return errors instead of falling back to a default value.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::num::ParseIntError;
use std::str::FromStr;
use std::num::IntErrorKind;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}



// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts : Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let part_name = parts.get(0);
        let part_age = parts.get(1);

        let mut nameTmp = "".to_string();
        let mut ageTmp = 0;

        if let Some(name) = part_name {
            if !name.is_empty() {
                nameTmp = name.to_string();
            }else{
                return Err(ParsePersonError::NoName);
            }
        } 

        if let Some(age_str) = part_age {
            // if !age_str.is_empty() {
                
            //     // if let Ok(age) = age_parse {
            //     //     ageTmp = age;
            //     // }else if Err(e) = age_parse {
            //     //     return Err(ParsePersonError::ParseInt(ParseIntError{kind : e}));
            //     // }
            // }
            let age_parse = age_str.parse::<usize>().map_err(ParsePersonError::ParseInt)?;
            ageTmp = age_parse;
        }

        // if nameTmp != "" && ageTmp != 0 {
        //     return Ok(Person{name : nameTmp, age : ageTmp});
        // }
        // return Ok(default);
        return Ok(Person{name : nameTmp, age : ageTmp});

        //https://users.rust-lang.org/t/rustlings-from-str-rs-matching-conditions/69558/7
        // if s.is_empty() {
        //     return Err(ParsePersonError::Empty);
        // }
    
        // let mut fields = s.split(',');

        // if let Some(first_field) = fields.next() {
        // if first_field.is_empty() {
        //     return Err(ParsePersonError::NoName);
        // }

        // if let Some(second_field) = fields.next() {
        //     // Check for a third field.
        //     if let Some(_) = fields.next() {
        //         return Err(ParsePersonError::BadLen);
        //     };

        //     let age = second_field.parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        //     return Ok(Person {
        //         name: first_field.into(),
        //         age
        //     });
        // } else {
        //     return Err(ParsePersonError::BadLen);
        // };

        // return Err(ParsePersonError::BadLen);
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
