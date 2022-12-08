use std::fs;
use slab_tree::*;

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
}

// to save an input file command, i need the context of the command
#[derive(Debug)]
struct CommandInput {
    command: ConsoleCommand,
    param: Option<String>,
    output: Option<String>, // cd returns nothing, ls *in theory* could return nothing
    cwd: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ConsoleCommand {
    Ls,
    Cd,
    Unknown,
}

pub fn day_7_main() {
    // https://adventofcode.com/2022/day/7
    let file_path = "inputs/day_7_demo.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    //println!("\n{}\n", input);
    
    // oof this is getting hard
    // parse the input to get a file system overview, and add up all the directories that weight at most 10.000

    // fisrt... get an idea of what data structures will hold the data. is it a tree for the fs? i guess so
    // each node could contain a struct with the data, an array pointing to the folders and an array with the files it contains
    // let's search for how to implement trees in rust before doing something too custom
    // ok the library slab_tree seems good

    // idea: each node contains a vec with file structs representing the files, directories are just naturally linked via tree node
    // wait that's not how it works, rework:
    // ROOT NODE -> struct called folder that contains the folder name and the files list, which is a vec of file struct, this good?

    // TOMORROW: 
    // test this tree model
    // decide how to parse the input: read commands and save them, and execute them later, or read them execute them jit

    // 1st might be better because ls in the input file makes things messy, tomorrow i'll decide

    let root_folder_test = Folder {
        name: "/".to_string(),
        files: vec![File { name: "poopsock.img".to_string(), size: 100 }]
    };

    let mut tree = TreeBuilder::new().with_root(root_folder_test).build();

    let folder_2 = Folder {
        name: "I am in level 2".to_string(),
        files: vec![File { name: "more_poop.img".to_string(), size: 1000}, File {name: "A folder".to_string(), size: 500}]
    };

    let root_id = tree.root_id().expect("No root in this tree");
    let mut root_node = tree.get_mut(root_id).expect("Root not found somehow");

    println!("{:?}", root_node.data());

    root_node.append(folder_2);

    println!("{:?}", root_node.first_child().unwrap().data());

    // seems ok, I'll worry about how to access it later, but doing a for on each node using "first child" and "next sibling"
    // and comparing the folder names should do the job. now it's time to parse the input, hardest part: how to save its data
    // in the long run saving its contents beforehand should be easier, i'll do that
    let command_history = parse_command_history(input);
    println!("Obtained the following commands:");
    for command in &command_history {
        println!("{:?}", command);
    }
    // parse complete B), only question is if cwd should contain the cwd AFTER the command or BEFORE

    let filesys_tree = build_file_sys_tree(command_history);


}

fn parse_command_history (input: String) -> Vec<CommandInput> {
    let mut commands_history: Vec<CommandInput> = vec![];
    let mut cwd: Vec<String> = vec![]; // starts from root regardless of the first command
    // separate the line in $, with this each line shuould be a different command
    for line in input.split("$ ").map(|str| str.strip_suffix('\n').unwrap_or(str)).collect::<Vec<&str>>() {
        println!("line is: {line}");

        // get the command from the line
        let command = match line.split(&[' ', '\n', '\r']).collect::<Vec<&str>>()[0] {
            "cd" => ConsoleCommand::Cd,
            "ls" => ConsoleCommand::Ls,
            _ => ConsoleCommand::Unknown,
        };

        // get the command from the line, supports only 1 but that's the relevant one for cd in this exercise
        let (parameter, output) = match command {
            ConsoleCommand::Cd => {
                (Some(line.split(&['\n', '\r']).collect::<Vec<&str>>()[0].split(' ').collect::<Vec<&str>>()[1].to_string()), None)
            },
            ConsoleCommand::Ls => (None, Some(line.split_once('\n').unwrap().1.to_string())),
            ConsoleCommand::Unknown => (None, None)
        };

        // save the command to the vec history
        if command != ConsoleCommand::Unknown {
            let command_detailed = CommandInput {
                command: command,
                param: parameter.clone(),
                output,
                cwd: cwd.clone(),
            };
            println!("Saving new command:{:?}", command_detailed);
            commands_history.push(command_detailed);
        }
        
        // apply the new cwd after savind the command
        if command == ConsoleCommand::Cd {
            match parameter.clone() {
                Some(p) => {
                    match &p[..] {
                        "/"  => {
                            cwd = vec![];
                        }
                        ".." => {
                            cwd.pop().expect("Tried to 'cwd ..' in root!");
                        },
                        _    => {
                            cwd.push(p.to_string());
                        }
                    }
                },
                None => println!("No parameter provided for 'cd'."),
            }
        }

        println!("--------------");
    }

    commands_history
}

fn build_file_sys_tree(commands: Vec<CommandInput>) -> Tree<Folder> {

    // time to build the tree
    let root_folder = Folder {
        name: "/".to_string(),
        files: vec![]
    };

    println!("File system build start...");
    let mut file_sys = TreeBuilder::new().with_root(root_folder).build();
    for command in commands {
        println!("Running: {:?}", command);
        
/* 
        // i will enter the tree from its root for every command which isn't very efficient but i've never worked with trees before so i'll keep it simple
        for folder in command.cwd {

            let mut new_folder = "".to_string();
            let mut curr_node = file_sys.root_id().expect("root doesn't exist");
            for child_folder in file_sys.root_mut().expect("root doesn't exist").as_ref().children(){

                if child_folder.data().name == folder {
                    println!("Found the folder -> {}", folder);
                    new_folder = folder.clone();
                    break;
                }

            }

            // did not find folder so we create it
            if new_folder == "" {
                println!("Could not find folder: {}", folder);

            }
            
        }
        */

        // jump to cwd, we assume that 'cd' and 'ls' create the appropiate node
        let mut curr_node: NodeId = file_sys.root_id().expect("root doesn't exist");
        for folder in command.cwd {
            for child_folder in file_sys.root_mut().expect("root doesn't exist").as_ref().children(){ 

                println!("searching for: {:?}", folder);

                if child_folder.data().name == folder {
                    println!("Found the folder -> {}", folder);
                    curr_node = child_folder.node_id();
                    break;
                }

            }

        };

        match command.command {
            ConsoleCommand::Cd => {
                
                // return to root
                if command.param.clone().unwrap_or(" ".to_string()) == "/" {
                    println!("Returned to root.");
                }

                // check if folder exists, if it doesn't, add to tree
                for child in file_sys.get(curr_node).unwrap().children() {
                    if command.param == Some(child.data().name.clone()) {
                        println!("Found folder requested for cd -> {}", command.param.clone().unwrap_or("".to_string()));
                    }
                }
                println!("Could NOT find folder requested for cd -> {}", command.param.unwrap_or("".to_string()));
                
            },
            ConsoleCommand::Ls => {

            },
            ConsoleCommand::Unknown => {}
        }
    }

    file_sys
}