extern crate cgol;
extern crate ctrlc;
extern crate ncurses;
extern crate rand;
extern crate unicode_segmentation;

use rand::{Rng, thread_rng};
use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use unicode_segmentation::UnicodeSegmentation;

fn handle_screen_resize(screen: ncurses::WINDOW, height: i32, width: i32) -> (i32, i32) {
    let resized = ncurses::is_term_resized(height, width);
    if resized {
        let width = ncurses::getmaxx(screen);
        let height = ncurses::getmaxy(screen);
        ncurses::wresize(screen, height, width);
        ncurses::refresh();
        ncurses::wrefresh(screen);
        return (height, width)
    }
    (height, width)
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let locale_conf = ncurses::LcCategory::all;
    ncurses::setlocale(locale_conf, "");

    let screen = ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let mut width = ncurses::getmaxx(screen);
    let mut height = ncurses::getmaxy(screen);
    let mut foo = handle_screen_resize(screen, height, width);
    height = foo.0;
    width = foo.1;

    let pause = time::Duration::from_millis(250);

    let choices = [cgol::Cell::Alive, cgol::Cell::Dead];
    let mut rng = thread_rng();

    let cells: Vec<cgol::Cell> = (0..32 * 32)
        .map(|_|  *rng.choose(&choices).unwrap())
        .collect();

    let mut universe = cgol::Universe::new(32, 32, cells);

    while running.load(Ordering::SeqCst) {
        foo = handle_screen_resize(screen, height, width);
        height = foo.0;
        width = foo.1;

        universe.tick();

        let render: String = universe.render();
        let lines = render.lines().take(height as usize);
        for (i, row) in lines.enumerate() {
            let row: String = UnicodeSegmentation::graphemes(row, true)
                .take(width as usize).collect();
            ncurses::mvwaddstr(screen, i as i32, 0, &row);
        }
        ncurses::wrefresh(screen);
        ncurses::refresh();

        thread::sleep(pause);
    }
    ncurses::endwin();
}
