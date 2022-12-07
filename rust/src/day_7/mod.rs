use std::collections::HashMap;

pub mod input;

pub fn day_7_part_1(input: &'static str) -> String {
    let file_system = parse_input_to_file_system(input);

    let mut directory_sizes = file_system
        .values()
        .map(|dir| dir.total_directory_size(&file_system))
        .collect::<Vec<_>>();

    directory_sizes.retain(|size| *size <= 100000);

    let answer = directory_sizes.iter().sum::<i32>();

    answer.to_string()
}

const FILE_SYSTEM_SIZE: i32 = 70000000;
const SPACE_REQUIRED: i32 = 30000000;

pub fn day_7_part_2(input: &'static str) -> String {
    let file_system = parse_input_to_file_system(input);

    let root: DirectoryPath = vec!["/".to_string()];

    let root_dir = file_system.get(&root).expect("Root should exist");

    let space_used = root_dir.total_directory_size(&file_system);

    let space_available = FILE_SYSTEM_SIZE - space_used;

    let minimum_delete_size = SPACE_REQUIRED - space_available;

    dbg!(
        "{} {} {}",
        space_available,
        SPACE_REQUIRED,
        minimum_delete_size
    );

    let mut valid_options = file_system
        .values()
        .filter_map(|dir| {
            let dir_size = dir.total_directory_size(&file_system);

            if dir_size > minimum_delete_size {
                Some(dir_size)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    valid_options.sort();

    dbg!("{:?}", &valid_options);

    valid_options
        .get(0)
        .expect("Should have valid answer")
        .to_string()
}

pub fn parse_input_to_file_system(input: &'static str) -> FileSystem {
    let mut file_system: FileSystem = HashMap::new();

    let mut current_path: DirectoryPath = vec!["/".to_string()];

    file_system.insert(
        current_path.clone(),
        Directory {
            name: "".to_string(),
            path: current_path.clone(),
            files: vec![],
            directories: vec![],
        },
    );

    for line in input.lines() {
        let trimmed_line = line.trim();

        if trimmed_line == "" {
            continue;
        }

        if is_command(trimmed_line) {
            let Command {
                command_type,
                argument,
            } = parse_command(trimmed_line);
            match command_type {
                CommandType::Cd => match argument {
                    Some(arg) => match arg {
                        Argument::Back => {
                            // Don't pop the path if you are already at the root
                            if current_path.len() > 1 {
                                current_path.pop();
                                current_path.pop();
                            }
                        }
                        Argument::Forward(path_string) => {
                            current_path.push(path_string);
                            current_path.push("/".to_string());
                        }
                        Argument::Root => current_path = vec!["/".to_string()],
                    },
                    None => unreachable!("Cd with no argument is invalid"),
                },
                CommandType::Ls => {
                    // Do nothing
                }
            };
        } else if is_directory(trimmed_line) {
            let directory = parse_directory(trimmed_line, current_path.clone());

            let mut new_dir_path = current_path.clone();
            new_dir_path.push(directory.name.clone());
            new_dir_path.push("/".to_string());
            file_system.insert(new_dir_path.clone(), directory);

            file_system
                .entry(current_path.clone())
                .and_modify(|dir| dir.directories.push(new_dir_path));
        } else {
            let file = parse_file(trimmed_line);
            file_system
                .entry(current_path.clone())
                .and_modify(|dir| dir.files.push(file));
        }
    }

    file_system
}

fn is_command(str: &str) -> bool {
    str.starts_with("$")
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    argument: Option<Argument>,
}

#[derive(Debug)]
enum CommandType {
    Cd,
    Ls,
}

#[derive(Debug)]
enum Argument {
    Back,
    Forward(String),
    Root,
}

fn parse_command(command: &str) -> Command {
    let stripped_command = command.strip_prefix("$ ").expect("Not an actual command");

    let mut command_split = stripped_command.split(" ");

    let command_name = command_split.next().expect("Unable to parse command term");

    let command_type: CommandType = match command_name {
        "cd" => CommandType::Cd,
        "ls" => CommandType::Ls,
        _ => unreachable!(),
    };

    let command_argument_text = command_split.next();

    let argument = match (&command_type, command_argument_text) {
        (CommandType::Cd, Some("..")) => Some(Argument::Back),
        (CommandType::Cd, Some("/")) => Some(Argument::Root),
        (CommandType::Cd, Some(directory)) => Some(Argument::Forward(directory.to_string())),
        (CommandType::Ls, None) => None,
        _ => unreachable!(),
    };

    let command = Command {
        command_type,
        argument,
    };

    command
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: i32,
}

fn is_directory(str: &str) -> bool {
    str.strip_prefix("dir ").is_some()
}

fn parse_file(str: &str) -> File {
    let file_text = str.split_once(" ").expect("Failed to parse file");

    File {
        size: i32::from_str_radix(file_text.0, 10).expect("Failed to parse file size"),
        name: file_text.1.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub path: DirectoryPath,
    pub files: Vec<File>,
    pub directories: Vec<DirectoryPath>,
}

impl Directory {
    pub fn total_file_size(&self) -> i32 {
        self.files.iter().fold(0, |acc, curr| acc + curr.size)
    }

    pub fn total_directory_size(&self, file_system: &FileSystem) -> i32 {
        let files_total_size = self.total_file_size();
        let nested_directory_size = self.directories.iter().fold(0, |acc, curr| {
            acc + file_system
                .get(curr)
                .expect("Directory should exist in file system")
                .total_directory_size(file_system)
        });

        files_total_size + nested_directory_size
    }
}

fn parse_directory(str: &str, current_path: DirectoryPath) -> Directory {
    Directory {
        name: str
            .strip_prefix("dir ")
            .expect("Failed to parse directory")
            .to_string(),
        path: current_path,
        files: Vec::new(),
        directories: Vec::new(),
    }
}

type FileSystem = HashMap<DirectoryPath, Directory>;

type DirectoryPath = Vec<String>;

#[cfg(test)]
mod test {

    use super::*;

    static TEST_INPUT: &str = r#"
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "#;

    #[test]
    fn part_1() {
        let answer = day_7_part_1(TEST_INPUT);

        assert_eq!(answer, "95437".to_string());
    }

    #[test]
    fn part_2() {
        let answer = day_7_part_2(TEST_INPUT);

        assert_eq!(answer, "24933642".to_string());
    }
}
