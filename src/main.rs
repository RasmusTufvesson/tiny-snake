use std::{collections::VecDeque, time::{Duration, Instant}};
use rand::Rng;
use sdl2::{event::Event, keyboard::Scancode, pixels::Color, rect::Point};

const FRAME_DELAY: u32 = 1_000_000_000 / 60;
const MOVE_DELAY: f64 = 0.1;

const PLAY_AREA: (u32, u32) = (30, 30);
const WINDOW_SIZE: (u32, u32) = (PLAY_AREA.0 / 3, PLAY_AREA.1);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Tiny Snake", WINDOW_SIZE.0, WINDOW_SIZE.1)
        .position_centered()
        .borderless()
        .input_grabbed()
        .build()
        .unwrap();

    sdl_context.mouse().show_cursor(false);

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();
    let mut last_frame = Instant::now();
    let mut rng = rand::rng();

    let mut segments = VecDeque::new();
    segments.push_front((PLAY_AREA.0 / 2, PLAY_AREA.1 / 2));
    let mut direction = (0, 0);
    let mut move_timer = MOVE_DELAY;
    let mut apple = (rng.random_range(0..PLAY_AREA.0), rng.random_range(0..PLAY_AREA.1));
    let mut input_queue = VecDeque::new();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let this_frame = Instant::now();
        let delta_time = (this_frame - last_frame).as_secs_f64();
        last_frame = this_frame;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(code), .. } => {
                    match code {
                        Scancode::W | Scancode::Up => {
                            input_queue.push_back((0, -1));
                        }
                        Scancode::S | Scancode::Down => {
                            input_queue.push_back((0, 1));
                        }
                        Scancode::A | Scancode::Left => {
                            input_queue.push_back((-1, 0));
                        }
                        Scancode::D | Scancode::Right => {
                            input_queue.push_back((1, 0));
                        }
                        Scancode::Escape => {
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        if let Some(dir) = input_queue.pop_front() {
            direction = dir;
        }

        move_timer -= delta_time;
        if move_timer <= 0.0 {
            move_timer = MOVE_DELAY;
            let (x, y) = (segments[0].0 as i32 + direction.0, segments[0].1 as i32 + direction.1);
            if x as u32 != apple.0 || y as u32 != apple.1 {
                segments.pop_back().unwrap();
            } else {
                apple = (rng.random_range(0..PLAY_AREA.0), rng.random_range(0..PLAY_AREA.1));
            }
            if x >= 0 && x < PLAY_AREA.0 as i32 && y >= 0 && y < PLAY_AREA.1 as i32 && !segments.contains(&(x as u32, y as u32)) {
                segments.push_front((x as u32, y as u32));
            } else {
                segments.clear();
                segments.push_front((PLAY_AREA.0 / 2, PLAY_AREA.1 / 2));
                direction = (0,0);
                apple = (rng.random_range(0..PLAY_AREA.0), rng.random_range(0..PLAY_AREA.1));
            }
        }

        let mut screen = [[[false; 3]; WINDOW_SIZE.0 as usize]; WINDOW_SIZE.1 as usize];

        for segment in &segments {
            screen[segment.1 as usize][segment.0 as usize / 3][segment.0 as usize % 3] = true;
        }
        screen[apple.1 as usize][apple.0 as usize / 3][apple.0 as usize % 3] = true;

        for y in 0..WINDOW_SIZE.1 {
            for x in 0..WINDOW_SIZE.0 {
                let rgb = &screen[y as usize][x as usize];
                if rgb[0] || rgb[1] || rgb[2] {
                    canvas.set_draw_color(Color::RGB(if rgb[0] { 255 } else { 0 }, if rgb[1] { 255 } else { 0 }, if rgb[2] { 255 } else { 0 }));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                }
            }
        }

        canvas.present();
        let diff = (last_frame - Instant::now()).as_nanos() as u32;
        if diff < FRAME_DELAY {
            ::std::thread::sleep(Duration::new(0, FRAME_DELAY - diff));
        }
    }
}
