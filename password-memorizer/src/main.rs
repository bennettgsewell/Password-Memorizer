use std::{time::Duration, io::{self, Write, Stdout}, fs::{self, File}, path::Path};
use crossterm::{self, terminal::{self, ClearType}, queue, execute};

fn read_line(stdin: &io::Stdin, expect_msg: &str) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf)
        .expect(expect_msg);
    buf.trim().to_string()
}

fn main() -> Result<(), io::Error>
{
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    terminal::disable_raw_mode()?;
    queue!(stdout, terminal::Clear(ClearType::FromCursorUp))?;
    queue!(stdout, crossterm::cursor::MoveTo(0, 0))?;
    stdout.flush()?;

    loop
    {
        println!("Type filename of password or \"exit\" to quit.");
        let filename = read_line(&stdin, "Failed to read filename from input.");

        if filename == "exit" {
            return Ok(());
        }
        
        //Creates password.txt
        let filepath = Path::new(&filename);
        if !filepath.exists() {
            if filepath.is_dir() {
                panic!("That is a directory, can't create a file there!");
            }

            // If the parent folder of the file does not exist, create it.
            if let Some(parent_path) = filepath.parent() {
                if !parent_path.exists() {
                    fs::create_dir_all(parent_path)
                        .expect("Failed to create the parent folder for the new file!");
                }
            }

            // Create the file
            File::create(&filepath)
                .expect("Failed to create the file!");

            // The user needs to enter some data in it first.
            println!("Created {}, please put in the password.", &filename);
            return Ok(());
        }
        
        //Read password.txt
        let password = fs::read_to_string(&filename)
            .expect("Failed to read the file!");
        let password = password.trim();
        if password.is_empty() {
            println!("{}, cannot be an empty file.", &filename);
            return Ok(());
        }
        
        //Test password memory.
        println!("Try your best!");
        
        let mut correct_entries: u64 = 0;
        
        loop
        {
            queue!(stdout, terminal::Clear(ClearType::FromCursorUp))?;
            queue!(stdout, crossterm::cursor::MoveTo(0, 0))?;
            stdout.flush()?;

            if correct_entries == 3 {
                break;
            }
            if correct_entries > 0 {
                println!("Correctly entered {correct_entries}/3");
            }
            if password_test(&password, &mut stdout)? {
                correct_entries += 1;
            } else {
                correct_entries = 0;
            }

            terminal::disable_raw_mode()?;
        }
    }
}

/// Tests users memory of the password.
fn password_test(password: &str, stdout: &mut Stdout) -> Result<bool, io::Error> {
    terminal::enable_raw_mode()?;

    for i in 0..password.len() {
        queue!(stdout, terminal::Clear(ClearType::FromCursorUp))?;
        queue!(stdout, crossterm::cursor::MoveTo(0, 0))?;

        // The password guessed so far
        if i > 0 {
            stdout.write(&password[0..i
                ].as_bytes())?;
        }

        stdout.write("_".as_bytes())?;

        stdout.flush()?;

        // Character that the user has to guess
        let guess_me = password.chars().nth(i).unwrap();
        let entered_char: char;

        // Loop terminal events until a key is pressed
        loop {
            if let crossterm::event::Event::Key(pressed) = crossterm::event::read()? {
                if let crossterm::event::KeyCode::Char(character_pressed) = pressed.code {
                    entered_char = character_pressed;
                    break;
                }
            }
        }

        if entered_char != guess_me {
            return Ok(false);
        }

        execute!(stdout, crossterm::cursor::MoveLeft(1))?;
        print!("{}", guess_me);
    }

    execute!(stdout, crossterm::cursor::MoveToColumn(0))?;
    execute!(stdout, crossterm::cursor::MoveDown(1))?;
    println!("Great work!!!");
    std::thread::sleep(Duration::from_secs(2));
    Ok(true)
}