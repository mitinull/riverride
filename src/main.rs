use std::io::stdout;

use crossterm::{cursor::MoveTo, style::Print, ExecutableCommand};

fn main() -> std::io::Result<()> {

    // init the screen
    let mut sc = stdout();
    sc.execute(MoveTo(0, 0))?;
    sc.execute(Print("Styled text here."))?;

    // init the game

    // game loop
    loop {
        // read and apply keyboard

        // physics()

        // draw
    }

    Ok(())
}
