use indoc::indoc;
use std::{io::{self, Stdout, Write}, process, thread::sleep, time::Duration};
use crossterm::{cursor, style::Stylize, terminal, ExecutableCommand, QueueableCommand};

struct Animation<'a> {
    states: &'a [&'a str],
    frames: usize,
    current_frame: usize,
    position: (u16, u16),
}

impl<'a> Animation<'a> {
    const fn new(position: (u16, u16), states: &'a [&str]) -> Self {
        let frames = states.len();

        Self {
            states,
            frames,
            current_frame: 0,
            position,
        }
    }

    fn increment_current_frame(&mut self) {
        if self.current_frame == self.frames - 1 {
            self.current_frame = 0;
        } else {
            self.current_frame += 1;
        }
    }
}

// Eyes for when important characters die, flipping between the eight judgements
// Also could be used for mysterious important characters with more of an
// influence on the story.
const DYING_ANIMATION: Animation = Animation::new((0, 0), &[
    indoc! {"
    ┌───────────┐
    │     |     │
    │     |     │
    │     *     │
    │    / \\    │
    │           │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │   \\       │
    │    \\      │
    │     *--   │
    │     |     │
    │           │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │           │
    │      /    │
    │ ----*     │
    │      \\    │
    │           │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │           │
    │     |     │
    │     *--   │
    │    /      │
    │   /       │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │           │
    │    \\ /    │
    │     *     │
    │     |     │
    │     |     │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │           │
    │     |     │
    │   --*     │
    │      \\    │
    │       \\   │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │           │
    │    \\      │
    │     *---- │
    │    /      │
    │           │
    └───────────┘
    "},
    indoc! {"
    ┌───────────┐
    │       /   │
    │      /    │
    │   --*     │
    │     |     │
    │           │
    └───────────┘
    "},
]);

// fn clear(stdout_handle: &mut Stdout, previous_state_height: u16) {
//     let current_row = cursor::position().unwrap().1;
//     for row in (current_row - previous_state_height)..=current_row {
//         let width = terminal::size().unwrap().0;
//         stdout_handle.queue(cursor::MoveTo(0, row)).unwrap();
//         stdout_handle.write_all(&vec![b' '; width as usize]).unwrap();
//     }
//     stdout_handle.queue(cursor::MoveTo(0, current_row - previous_state_height)).unwrap();
// }

fn run_animations(animations: &mut [Animation], delay: Duration, stdout_handle: &mut Stdout) -> ! {
    loop {
        // Place each animation and each frame of the animation
        for animation in animations.iter_mut() {
            for (index, line) in animation.states[animation.current_frame].lines().enumerate() {
                stdout_handle.queue(cursor::MoveTo(animation.position.0, animation.position.1 + index as u16)).unwrap();
                stdout_handle.write_all(line.as_bytes()).unwrap();
            }

            stdout_handle.flush().unwrap();
            animation.increment_current_frame();
            sleep(delay);
        }
    }
}

// The unwrapping is made under the assumption that it would have errored
// earlier if it will have errored.
fn main() {
    // Set the terminal title
    let mut stdout_handle = io::stdout();
    let _ = stdout_handle.execute(terminal::SetTitle("Webcomic concept art!"));

    // Move to a blank screen
    if let Err(e) = stdout_handle.execute(terminal::EnterAlternateScreen) {
        eprintln!("{}", "Your terminal doesn't support alternate screens!".red());
        eprintln!("{}", "The program might not work as intended.".red());
        eprintln!("{} {}", "Exact error:".dark_grey(), e.to_string().dark_grey());
    }

    // Make cursor visble when animation is killed.
    if let Err(e) = ctrlc::set_handler(|| {
        let mut stdout_handler = io::stdout();
        stdout_handler.execute(cursor::Show).unwrap();
        stdout_handler.execute(terminal::LeaveAlternateScreen).unwrap();
        process::exit(1);
    }) {
        eprintln!("{}", "Failed to set ctrl + c handler!".red());
        eprintln!("Continuing would cause your cursor to be outside of this process invisble.");
        eprintln!("{} {}", "Exact error:".dark_grey(), e.to_string().dark_grey());
        return;
    }

    // Make cursor invisble on startup for animations.
    if let Err(e) = stdout_handle.queue(cursor::Hide) {
        eprintln!("{}", "Queue to stdout! Your terminal is most likely unusable!".red());
        eprintln!("{} {}", "Exact error:".dark_grey(), e.to_string().dark_grey());
        return;
    }

    run_animations(&mut [DYING_ANIMATION], Duration::from_millis(150), &mut stdout_handle)

    // loop {
    //     for state in DYING_STATES.states {
    //         stdout_handle.write_all(state.as_bytes()).unwrap();
    //         stdout_handle.flush().unwrap();
    //         sleep(Duration::from_millis(150));
    //         clear(&mut stdout_handle, state.lines().count() as u16);
    //     }
    // }
}
