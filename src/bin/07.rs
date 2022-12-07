use std::collections::HashMap;

type Path = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Content {
    File(usize),
    Directory(Directory),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Directory {
    contents: HashMap<String, Content>,
}

impl Directory {
    fn new() -> Directory {
        let contents = HashMap::new();
        Directory { contents }
    }

    fn add_file(&mut self, path: &[String], filename: &str, size: usize) {
        match path.first() {
            Some(dir_name) => {
                if let Some(Content::Directory(directory)) = self.contents.get_mut(dir_name) {
                    directory.add_file(&path[1..], filename, size);
                }
            }
            None => {
                self.contents
                    .insert(filename.to_string(), Content::File(size));
            }
        }
    }

    fn add_directory(&mut self, path: &[String], name: &str) {
        match path.first() {
            Some(dir_name) => {
                if let Some(Content::Directory(directory)) = self.contents.get_mut(dir_name) {
                    directory.add_directory(&path[1..], name);
                }
            }
            None => {
                self.contents
                    .insert(name.to_string(), Content::Directory(Directory::new()));
            }
        }
    }

    fn size(&self, path: &[String]) -> usize {
        match path.first() {
            Some(dir_name) => {
                if let Some(Content::Directory(directory)) = self.contents.get(dir_name) {
                    directory.size(&path[1..])
                } else {
                    0
                }
            }
            None => self
                .contents
                .iter()
                .map(|(_, c)| match c {
                    Content::Directory(dir) => dir.size(path),
                    Content::File(size) => *size,
                })
                .sum(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct FileSystem {
    root_directory: Directory,
    directory_paths: Vec<Path>,
}

impl FileSystem {
    fn build(input: input_parser::Input) -> FileSystem {
        let mut path_marker: Path = Vec::new();
        let mut root_directory: Directory = Directory::new();
        let mut directory_paths: Vec<Path> = Vec::new();
        for line in input.lines {
            match line {
                input_parser::Line::Command(input_parser::Command::Cd(path)) => match path.as_str()
                {
                    "/" => {
                        path_marker.clear();
                    }
                    ".." => {
                        path_marker.pop();
                    }
                    _ => {
                        path_marker.push(path);
                    }
                },
                input_parser::Line::Command(input_parser::Command::Ls) => {}
                input_parser::Line::Output(input_parser::LsResult::File(input_parser::File {
                    filename,
                    size,
                })) => {
                    root_directory.add_file(&path_marker, &filename, size);
                }
                input_parser::Line::Output(input_parser::LsResult::Directory(
                    input_parser::Directory { name },
                )) => {
                    root_directory.add_directory(&path_marker, &name);

                    let mut new_directory = path_marker.clone();
                    new_directory.push(name.clone());
                    directory_paths.push(new_directory);
                }
            }
        }
        FileSystem {
            root_directory,
            directory_paths,
        }
    }

    fn sum_dir_sizes_below(&self, threshold: usize) -> usize {
        self.directory_paths
            .iter()
            .map(|path| self.root_directory.size(path))
            .filter(|size| *size <= threshold)
            .sum()
    }

    fn best_deletable_directory_size(
        &self,
        fs_capacity: usize,
        needed_space: usize,
    ) -> Option<usize> {
        let free_space = fs_capacity - self.root_directory.size(&[]);
        let needed_space = needed_space - free_space;
        self.directory_paths
            .iter()
            .map(|path| self.root_directory.size(path))
            .filter(|size| *size >= needed_space)
            .min()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let fs = FileSystem::build(input.parse::<input_parser::Input>().ok()?);
    Some(fs.sum_dir_sizes_below(100000))
}

pub fn part_two(input: &str) -> Option<usize> {
    let fs = FileSystem::build(input.parse::<input_parser::Input>().ok()?);
    fs.best_deletable_directory_size(70000000, 30000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

mod input_parser {
    use nom::{Finish, IResult};
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct File {
        pub filename: String,
        pub size: usize,
    }

    fn file(input: &str) -> IResult<&str, LsResult> {
        let parser = nom::sequence::separated_pair(
            nom::character::complete::digit1,
            nom::character::complete::space1,
            nom::character::complete::not_line_ending,
        );
        nom::combinator::map_opt(parser, |(size_s, filename): (&str, &str)| {
            let size: usize = size_s.parse().ok()?;
            let filename = filename.to_string();
            Some(LsResult::File(File { filename, size }))
        })(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Directory {
        pub name: String,
    }

    fn directory(input: &str) -> IResult<&str, LsResult> {
        let parser = nom::sequence::separated_pair(
            nom::bytes::complete::tag("dir"),
            nom::character::complete::space1,
            nom::character::complete::not_line_ending,
        );
        nom::combinator::map(parser, |(_, name): (&str, &str)| {
            let name = name.to_string();
            LsResult::Directory(Directory { name })
        })(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum LsResult {
        File(File),
        Directory(Directory),
    }

    fn ls_result(input: &str) -> IResult<&str, LsResult> {
        nom::branch::alt((file, directory))(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Command {
        Ls,
        Cd(String),
    }

    fn cd_command(input: &str) -> IResult<&str, Command> {
        let parser = nom::sequence::separated_pair(
            nom::bytes::complete::tag("cd"),
            nom::character::complete::space1,
            nom::character::complete::not_line_ending,
        );
        nom::combinator::map(parser, |(_cmd, path): (&str, &str)| {
            Command::Cd(path.to_string())
        })(input)
    }

    fn ls_command(input: &str) -> IResult<&str, Command> {
        nom::combinator::map(nom::bytes::complete::tag("ls"), |_| Command::Ls)(input)
    }

    fn command(input: &str) -> IResult<&str, Command> {
        let parser = nom::sequence::separated_pair(
            nom::bytes::complete::tag("$"),
            nom::character::complete::space1,
            nom::branch::alt((cd_command, ls_command)),
        );
        nom::combinator::map(parser, |(_, cmd)| cmd)(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Line {
        Command(Command),
        Output(LsResult),
    }

    fn line(input: &str) -> IResult<&str, Line> {
        nom::branch::alt((
            nom::combinator::map(command, Line::Command),
            nom::combinator::map(ls_result, Line::Output),
        ))(input)
    }

    fn program(input: &str) -> IResult<&str, Input> {
        let parser = nom::multi::separated_list1(nom::character::complete::newline, line);
        nom::combinator::map(parser, |lines| Input { lines })(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Input {
        pub lines: Vec<Line>,
    }

    impl FromStr for Input {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match program(s).finish() {
                Ok((_remaining, plan)) => Ok(plan),
                Err(nom::error::Error { input, code }) => Err(Self::Err {
                    input: input.to_string(),
                    code,
                }),
            }
        }
    }
}
