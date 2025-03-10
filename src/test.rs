use crate::parser::parse;
use crate::reader::PDFReader;
use crate::structure::{AnyPDFData, Object};
use std::fs::File;

#[cfg(test)]
pub mod pdf_tests {
    use super::*;
    /**
     * A note on the tests
     * 1) All of these tests have their expected outputs generated by AI; most
     * have their inputs also AI-genned. All outputs and inputs are reviewed.
     * 2) The test inputs are stored in a .pdf file - but they cannot be opened
     * in a PDF editor and are standard UTF-8/ANSI.
     */
    #[test]
    fn test_number_parsing() {
        let objects = parse(&mut PDFReader::from(
            File::open("tests/numtests.pdf").unwrap(),
        ))
        .unwrap()
        .objects;
        let expected = vec![
            Object {
                number: 1,
                gen: 0,
                data: AnyPDFData::Integer(123),
            },
            Object {
                number: 2,
                gen: 0,
                data: AnyPDFData::Integer(43445),
            },
            Object {
                number: 3,
                gen: 0,
                data: AnyPDFData::Integer(17),
            },
            Object {
                number: 4,
                gen: 0,
                data: AnyPDFData::Integer(-98),
            },
            Object {
                number: 5,
                gen: 0,
                data: AnyPDFData::Integer(0),
            },
            Object {
                number: 10,
                gen: 0,
                data: AnyPDFData::Real(34.5),
            },
            Object {
                number: 11,
                gen: 0,
                data: AnyPDFData::Real(-3.62),
            },
            Object {
                number: 12,
                gen: 0,
                data: AnyPDFData::Real(123.6),
            },
            Object {
                number: 13,
                gen: 0,
                data: AnyPDFData::Real(4.0),
            },
            Object {
                number: 14,
                gen: 0,
                data: AnyPDFData::Real(-0.002),
            },
            Object {
                number: 15,
                gen: 0,
                data: AnyPDFData::Real(0.0),
            },
        ];
        assert_eq!(expected.len(), objects.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], objects[i]);
        }
    }
    #[test]
    fn test_string_parsing() {
        let objects = parse(&mut PDFReader::from(
            File::open("tests/strtests.pdf").unwrap(),
        ))
        .unwrap()
        .objects;

        let expected =
            vec![
                Object {
                    number: 1,
                    gen: 0,
                    data: AnyPDFData::String("This is a string".to_string()),
                },
                Object {
                    number: 2,
                    gen: 0,
                    data: AnyPDFData::String("Strings can contain newlines\nand such.".to_string()),
                },
                Object {
                    number: 3,
                    gen: 0,
                    data: AnyPDFData::String("Strings can contain balanced parentheses ()\nand special characters ( * ! & } ^ %and so on) .".to_string()),
                },
                Object {
                    number: 4,
                    gen: 0,
                    data: AnyPDFData::String("The following is an empty string .".to_string()),
                },
                Object {
                    number: 5,
                    gen: 0,
                    data: AnyPDFData::String("".to_string()),
                },
                Object {
                    number: 6,
                    gen: 0,
                    data: AnyPDFData::String("It has zero (0) length.".to_string()),
                },
                Object {
                    number: 7,
                    gen: 0,
                    data: AnyPDFData::String("These two strings are the same.".to_string()),
                },
                Object {
                    number: 8,
                    gen: 0,
                    data: AnyPDFData::String("These two strings are the same.".to_string()),
                },
                Object {
                    number: 9,
                    gen: 0,
                    data: AnyPDFData::String("This string has an end-of-line at the end of it.\n".to_string()),
                },
                Object {
                    number: 10,
                    gen: 0,
                    data: AnyPDFData::String("So does this one.\n".to_string()),
                },
                Object {
                    number: 11,
                    gen: 0,
                    data: AnyPDFData::String("This string contains \u{a5}two octal characters\u{c7}.".to_string()),
                },
                Object {
                    number: 20,
                    gen: 0,
                    data: AnyPDFData::String("Nov shmoz ka pop.".to_string()),
                }
            ];
        assert_eq!(expected.len(), objects.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], objects[i]);
        }
    }
    #[test]
    fn test_name_parsing() {
        let objects = parse(&mut PDFReader::from(
            File::open("tests/nametests.pdf").unwrap(),
        ))
        .unwrap()
        .objects;
        let expected = vec![
            Object {
                number: 1,
                gen: 0,
                data: AnyPDFData::Name("Name1".to_string()),
            },
            Object {
                number: 2,
                gen: 0,
                data: AnyPDFData::Name("ASomewhatLongerName".to_string()),
            },
            Object {
                number: 3,
                gen: 0,
                data: AnyPDFData::Name("A;Name_With-Various***Characters?".to_string()),
            },
            Object {
                number: 4,
                gen: 0,
                data: AnyPDFData::Name("1.2".to_string()),
            },
            Object {
                number: 5,
                gen: 0,
                data: AnyPDFData::Name("$$".to_string()),
            },
            Object {
                number: 6,
                gen: 0,
                data: AnyPDFData::Name("@pattern".to_string()),
            },
            Object {
                number: 7,
                gen: 0,
                data: AnyPDFData::Name(".notdef".to_string()),
            },
            Object {
                number: 8,
                gen: 0,
                data: AnyPDFData::Name("Lime Green".to_string()),
            },
            Object {
                number: 9,
                gen: 0,
                data: AnyPDFData::Name("paired()parentheses".to_string()),
            },
            Object {
                number: 10,
                gen: 0,
                data: AnyPDFData::Name("The_Key_of_F#_Minor".to_string()),
            },
            Object {
                number: 11,
                gen: 0,
                data: AnyPDFData::Name("AB".to_string()),
            },
        ];
        assert_eq!(expected.len(), objects.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], objects[i]);
        }
    }
    #[test]
    fn test_array_parsing() {
        let objects = parse(&mut PDFReader::from(
            File::open("tests/arrtests.pdf").unwrap(),
        ))
        .unwrap()
        .objects;

        let expected = vec![
            Object {
                number: 1,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Integer(549),
                    AnyPDFData::Real(3.14),
                    AnyPDFData::Boolean(false),
                    AnyPDFData::String("Ralph".to_string()),
                    AnyPDFData::Name("SomeName".to_string()),
                ]),
            },
            Object {
                number: 2,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Name("Name1".to_string()),
                    AnyPDFData::Name("ASomewhatLongerName".to_string()),
                    AnyPDFData::Name("A;Name_With-Various***Characters?".to_string()),
                ]),
            },
            Object {
                number: 3,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::String("one".to_string()),
                    AnyPDFData::String("two".to_string()),
                    AnyPDFData::String("three".to_string()),
                    AnyPDFData::String("four".to_string()),
                    AnyPDFData::String("five".to_string()),
                ]),
            },
            Object {
                number: 4,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Name("ABC".to_string()),
                    AnyPDFData::Name("XYZ".to_string()),
                    AnyPDFData::Boolean(false),
                    AnyPDFData::Integer(123),
                    AnyPDFData::Real(3.14),
                ]),
            },
            Object {
                number: 5,
                gen: 0,
                data: AnyPDFData::Array(vec![]),
            },
            Object {
                number: 6,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Integer(1),
                    AnyPDFData::Integer(2),
                    AnyPDFData::Integer(3),
                    AnyPDFData::Integer(4),
                ]),
            },
            Object {
                number: 7,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Integer(1),
                    AnyPDFData::Integer(2),
                    AnyPDFData::Array(vec![AnyPDFData::Integer(3), AnyPDFData::Integer(4)]),
                    AnyPDFData::Integer(5),
                ]),
            },
            Object {
                number: 8,
                gen: 0,
                data: AnyPDFData::Array(vec![AnyPDFData::Array(vec![AnyPDFData::Array(vec![])])]),
            },
            Object {
                number: 9,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::String("hello".to_string()),
                    AnyPDFData::Integer(123),
                    AnyPDFData::Boolean(true),
                    AnyPDFData::Name("Name".to_string()),
                ]),
            },
            Object {
                number: 10,
                gen: 0,
                data: AnyPDFData::Array(vec![
                    AnyPDFData::Integer(1),
                    AnyPDFData::Integer(2),
                    AnyPDFData::Integer(3),
                ]),
            },
            Object {
                number: 11,
                gen: 0,
                data: AnyPDFData::Array(vec![]),
            },
            Object {
                number: 12,
                gen: 0,
                data: AnyPDFData::Array(vec![]),
            },
        ];
        assert_eq!(expected.len(), objects.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], objects[i]);
        }
    }
    #[test]
    fn test_dictionary_parsing() {
        let objects = parse(&mut PDFReader::from(
            File::open("tests/dictests.pdf").unwrap(),
        ))
        .unwrap()
        .objects;

        let expected = vec![
            Object {
                number: 1,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    ("Type".to_string(), AnyPDFData::Name("Example".to_string())),
                    (
                        "Subtype".to_string(),
                        AnyPDFData::Name("DictionaryExample".to_string()),
                    ),
                    ("Version".to_string(), AnyPDFData::Real(0.01)),
                    ("IntegerItem".to_string(), AnyPDFData::Integer(12)),
                    (
                        "StringItem".to_string(),
                        AnyPDFData::String("a string".to_string()),
                    ),
                    (
                        "Subdictionary".to_string(),
                        AnyPDFData::Dictionary(vec![
                            ("Item1".to_string(), AnyPDFData::Real(0.4)),
                            ("Item2".to_string(), AnyPDFData::Boolean(true)),
                            (
                                "LastItem".to_string(),
                                AnyPDFData::String("not !".to_string()),
                            ),
                            (
                                "VeryLastItem".to_string(),
                                AnyPDFData::String("OK".to_string()),
                            ),
                        ]),
                    ),
                ]),
            },
            Object {
                number: 2,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    ("Type".to_string(), AnyPDFData::Name("Example".to_string())),
                    ("Value".to_string(), AnyPDFData::Integer(123)),
                ]),
            },
            Object {
                number: 3,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    (
                        "Name".to_string(),
                        AnyPDFData::String("John Doe".to_string()),
                    ),
                    ("Age".to_string(), AnyPDFData::Integer(30)),
                    (
                        "City".to_string(),
                        AnyPDFData::String("New York".to_string()),
                    ),
                ]),
            },
            Object {
                number: 4,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    (
                        "Array".to_string(),
                        AnyPDFData::Array(vec![
                            AnyPDFData::Integer(1),
                            AnyPDFData::Integer(2),
                            AnyPDFData::Integer(3),
                        ]),
                    ),
                    (
                        "Dict".to_string(),
                        AnyPDFData::Dictionary(vec![(
                            "Inner".to_string(),
                            AnyPDFData::Name("Value".to_string()),
                        )]),
                    ),
                ]),
            },
            Object {
                number: 5,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    ("Boolean".to_string(), AnyPDFData::Boolean(true)),
                    ("Indirect".to_string(), AnyPDFData::ObjRef(10, 0)),
                ]),
            },
            Object {
                number: 6,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    ("Key1".to_string(), AnyPDFData::Name("Value1".to_string())),
                    ("Key2".to_string(), AnyPDFData::Name("Value2".to_string())),
                ]),
            },
            Object {
                number: 7,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![]),
            },
            Object {
                number: 8,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![(
                    "Key1".to_string(),
                    AnyPDFData::String("Value with (nested) parentheses".to_string()),
                )]),
            },
            Object {
                number: 9,
                gen: 0,
                data: AnyPDFData::Dictionary(vec![
                    ("Key1".to_string(), AnyPDFData::Integer(456)),
                    ("Key2".to_string(), AnyPDFData::Name("Value2".to_string())),
                ]),
            },
        ];
        println!("fff{:?}", objects);
        assert_eq!(expected.len(), objects.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], objects[i]);
        }
    }
}
