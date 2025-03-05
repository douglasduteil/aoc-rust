//

use nom::{
    Finish,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of, space1},
    combinator::{map, recognize},
    error::{VerboseError, context, convert_error},
    multi::{many1, separated_list0, separated_list1},
    number::complete::float,
    sequence::{preceded, separated_pair},
};
use std::str::FromStr;

//

pub struct TerminalOutput(pub Vec<Command>);

//

impl FromStr for TerminalOutput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_, commands)) => Ok(Self(commands)),
            Err(e) => Err(convert_error(s, e)),
        }
    }
}

//

type IResult<T, U> = nom::IResult<T, U, VerboseError<T>>;

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    context(
        "commands",
        separated_list1(
            newline,
            alt((map(command_cd, Command::Cd), map(command_ls, Command::Ls))),
        ),
    )(input)
}

#[cfg(test)]
mod parse_integration_tests {
    use super::*;

    #[test]
    fn test_parse_validates_specific_command_results() {
        // Create a simple input with both cd and ls commands
        let input = "$ cd /\n$ ls\ndir a\n123 b.txt\n$ cd a";

        // Parse the input
        let (_, commands) = parse(input).unwrap();

        // Verify we got the expected number of commands
        assert_eq!(commands.len(), 3);

        // Check the specific results of each command
        match &commands[0] {
            Command::Cd(Cd::RootDir) => {} // First command should be "cd /"
            _ => panic!("First command should be Cd::RootDir"),
        }

        match &commands[1] {
            Command::Ls(contents) => {
                assert_eq!(contents.len(), 2); // Should have 2 entries

                // Check directory entry
                if let Content::Directory(name) = &contents[0] {
                    assert_eq!(name, "a");
                } else {
                    panic!("First ls entry should be Directory");
                }
                match &contents[0] {
                    Content::Directory(name) => assert_eq!(name, "a"),
                    _ => panic!("First ls entry should be Directory"),
                }

                // Check file entry
                match &contents[1] {
                    Content::File { name, size } => {
                        assert_eq!(name, "b.txt");
                        assert_eq!(*size, 123);
                    }
                    _ => panic!("Second ls entry should be File"),
                }
            }
            _ => panic!("Second command should be Ls with correct contents"),
        }

        match &commands[2] {
            Command::Cd(Cd::In(dir_name)) => assert_eq!(dir_name, "a"), // Third command should be "cd a"
            _ => panic!("Third command should be Cd::In with 'a'"),
        }
    }
}

//

fn command_cd(input: &str) -> IResult<&str, Cd> {
    context(
        "cd",
        preceded(
            tag("$ cd "),
            alt((
                map(tag("/"), |_| Cd::RootDir),
                map(tag(".."), |_| Cd::Out),
                map(context("directory", alphanumeric1), |directory| {
                    Cd::In(String::from(directory))
                }),
            )),
        ),
    )(input)
}
#[cfg(test)]
mod command_cd_tests {
    use super::*;

    #[test]
    fn test_cd_root() {
        let input = "$ cd /";
        let (remaining, cd_command) = command_cd(input).unwrap();
        assert_eq!(remaining, "");

        match cd_command {
            Cd::RootDir => {}
            _ => panic!("Expected RootDir variant"),
        }
    }

    #[test]
    fn test_cd_out() {
        let input = "$ cd ..";
        let (remaining, cd_command) = command_cd(input).unwrap();
        assert_eq!(remaining, "");

        match cd_command {
            Cd::Out => {}
            _ => panic!("Expected Out variant"),
        }
    }

    #[test]
    fn test_cd_in() {
        let input = "$ cd example";
        let (remaining, cd_command) = command_cd(input).unwrap();
        assert_eq!(remaining, "");

        match cd_command {
            Cd::In(dir_name) => assert_eq!(dir_name, "example"),
            _ => panic!("Expected In variant"),
        }
    }

    #[test]
    fn test_cd_multiple_directories() {
        // Testing that the parser handles the first cd command correctly
        // and leaves the rest as remaining input
        let input = "$ cd example\n$ cd subdir";
        let (remaining, cd_command) = command_cd(input).unwrap();
        assert_eq!(remaining, "\n$ cd subdir");

        match cd_command {
            Cd::In(dir_name) => assert_eq!(dir_name, "example"),
            _ => panic!("Expected In variant"),
        }
    }

    #[test]
    fn test_cd_invalid_input() {
        // Test with invalid input - should return an error
        let input = "$ cd";
        let result = command_cd(input);
        assert!(result.is_err());
    }
}

fn command_ls(input: &str) -> IResult<&str, Vec<Content>> {
    context(
        "ls",
        preceded(
            tag("$ ls\n"),
            separated_list0(
                newline,
                alt((
                    // dir a
                    map(
                        separated_pair(tag("dir"), space1, is_filename),
                        |(_, dir)| Content::Directory(String::from(dir)),
                    ),
                    // 14848514 b.txt
                    map(
                        separated_pair(float, space1, is_filename),
                        |(size, name)| Content::File {
                            name: String::from(name),
                            size: size as usize,
                        },
                    ),
                )),
            ),
        ),
    )(input)
}

//

#[cfg(test)]
mod command_ls_tests {
    use super::*;

    #[test]
    fn test_empty_ls() {
        let input = "$ ls\n";
        let (remaining, contents) = command_ls(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(contents.len(), 0);
    }

    #[test]
    fn test_ls_with_directory() {
        let input = "$ ls\ndir example";
        let (remaining, contents) = command_ls(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(contents.len(), 1);

        match &contents[0] {
            Content::Directory(name) => assert_eq!(name, "example"),
            _ => panic!("Expected Directory variant"),
        }
    }

    #[test]
    fn test_ls_with_file() {
        let input = "$ ls\n123 test.txt";
        let (remaining, contents) = command_ls(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(contents.len(), 1);

        match &contents[0] {
            Content::File { name, size } => {
                assert_eq!(name, "test.txt");
                assert_eq!(*size, 123);
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_ls_with_multiple_entries() {
        let input = "$ ls\ndir folder1\n14848514 b.txt\ndir folder2\n8504156 c.dat";
        let (remaining, contents) = command_ls(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(contents.len(), 4);

        // Check directory entries
        match &contents[0] {
            Content::Directory(name) => assert_eq!(name, "folder1"),
            _ => panic!("Expected Directory variant"),
        }

        match &contents[2] {
            Content::Directory(name) => assert_eq!(name, "folder2"),
            _ => panic!("Expected Directory variant"),
        }

        // Check file entries
        match &contents[1] {
            Content::File { name, size } => {
                assert_eq!(name, "b.txt");
                assert_eq!(*size, 14848514);
            }
            _ => panic!("Expected File variant"),
        }

        match &contents[3] {
            Content::File { name, size } => {
                assert_eq!(name, "c.dat");
                assert_eq!(*size, 8504156);
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_ls_with_complex_filenames() {
        let input =
            "$ ls\n123 file-with-dashes.txt\n456 file_with_underscores.log\n789 file.with.dots";
        let (remaining, contents) = command_ls(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(contents.len(), 3);

        match &contents[0] {
            Content::File { name, size } => {
                assert_eq!(name, "file-with-dashes.txt");
                assert_eq!(*size, 123);
            }
            _ => panic!("Expected File variant"),
        }

        match &contents[1] {
            Content::File { name, size } => {
                assert_eq!(name, "file_with_underscores.log");
                assert_eq!(*size, 456);
            }
            _ => panic!("Expected File variant"),
        }

        match &contents[2] {
            Content::File { name, size } => {
                assert_eq!(name, "file.with.dots");
                assert_eq!(*size, 789);
            }
            _ => panic!("Expected File variant"),
        }
    }
}

//

fn is_filename(input: &str) -> IResult<&str, &str> {
    let allowed_chars = alt((alphanumeric1, recognize(one_of("._-"))));
    recognize(many1(allowed_chars))(input)
}

//

pub enum Command {
    Cd(Cd),
    Ls(Vec<Content>),
}

pub enum Cd {
    In(String),
    Out,
    RootDir,
}

#[allow(dead_code)]
pub enum Content {
    Directory(String),
    File { name: String, size: usize },
}
