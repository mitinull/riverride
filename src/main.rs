use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::{self, Duration},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::{thread_rng, Rng};

struct World {
    player_column: u16,
    player_line: u16,
    map: Vec<(u16, u16)>,
    player_is_dead: bool,
    next_left: u16,
    next_right: u16,
}

fn draw(mut sc: &Stdout, world: &World, max_column: &u16) -> std::io::Result<()> {
    sc.queue(Clear(ClearType::All))?;

    // draw the map
    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;

        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat((max_column - world.map[l].1) as usize)))?;
    }

    // draw the player
    sc.queue(MoveTo(world.player_column, world.player_line))?;
    sc.queue(Print("P"))?;

    sc.flush()?;

    Ok(())
}

fn physics(mut world: World) -> std::io::Result<World> {
    let mut rng = thread_rng();

    if world.player_column < world.map[world.player_line as usize].0
        || world.player_column >= world.map[world.player_line as usize].1
    {
        world.player_is_dead = true;
    }

    for l in (0..world.map.len() - 1).rev() {
        world.map[l + 1] = world.map[l];
    }
    if world.next_left > world.map[0].0 {
        world.map[0].0 += 1;
    }
    if world.next_left < world.map[0].0 {
        world.map[0].0 -= 1;
    }
    if world.next_right > world.map[0].1 {
        world.map[0].1 += 1;
    }
    if world.next_right < world.map[0].1 {
        world.map[0].1 -= 1;
    }

    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7 {
        world.next_left = rng.gen_range(world.next_left - 5..world.next_left + 5)
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7 {
        world.next_right = rng.gen_range(world.next_right - 5..world.next_right + 5)
    }
    if world.next_right - world.next_left < 3 {
        // todo: check abs
        world.next_right += 3;
    }

    Ok(world)
}

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (max_column, max_line) = size().unwrap();

    enable_raw_mode()?;
    sc.execute(Hide)?;

    // init the game
    let slowness = 50;
    let mut world = World {
        player_column: max_column / 2,
        player_line: max_line - 1,
        map: vec![(max_column / 2 - 5, max_column / 2 + 5); max_line as usize],
        player_is_dead: false,
        next_left: max_column / 2 - 6,
        next_right: max_column / 2 + 7,
    };

    // game loop
    while !world.player_is_dead {
        // read and apply keyboard
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            KeyCode::Char('w') => {
                                if world.player_line > 0 {
                                    world.player_line -= 1;
                                }
                            }
                            KeyCode::Char('s') => {
                                if world.player_line < max_line - 1 {
                                    world.player_line += 1;
                                }
                            }
                            KeyCode::Char('a') => {
                                if world.player_column > 0 {
                                    world.player_column -= 1;
                                }
                            }
                            KeyCode::Char('d') => {
                                if world.player_column < max_column - 1 {
                                    world.player_column += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        world = physics(world).unwrap();

        draw(&sc, &world, &max_column)?;

        thread::sleep(time::Duration::from_millis(slowness));
    }

    sc.execute(Show)?;
    disable_raw_mode()?;
    sc.execute(Clear(ClearType::All))?;
    sc.execute(Print("Thanks for playing"))?;

    Ok(())
}
