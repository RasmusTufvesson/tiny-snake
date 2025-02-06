use std::{collections::VecDeque, time::{Duration, Instant}};
use rand::Rng;
use sdl2::{event::Event, keyboard::Scancode, pixels::Color, rect::Point};

const FRAME_DELAY: u32 = 1_000_000_000u32 / 60;
const MOVE_DELAY: f64 = 0.1;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let play_area = (30, 30);
    let window_size = (play_area.0 / 3, play_area.1);

    let window = video_subsystem.window("Tiny Snake", window_size.0, window_size.1)
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
    segments.push_front((play_area.0 / 2, play_area.1 / 2));
    let mut direction = (0, 0);
    let mut move_timer = MOVE_DELAY;
    let mut apple = (rng.random_range(0..play_area.0), rng.random_range(0..play_area.1));

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
                            direction = (0, -1);
                        }
                        Scancode::S | Scancode::Down => {
                            direction = (0, 1);
                        }
                        Scancode::A | Scancode::Left => {
                            direction = (-1, 0);
                        }
                        Scancode::D | Scancode::Right => {
                            direction = (1, 0);
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
        
        move_timer -= delta_time;
        if move_timer <= 0.0 {
            move_timer = MOVE_DELAY;
            let (x, y) = (segments[0].0 as i32 + direction.0, segments[0].1 as i32 + direction.1);
            if x as u32 != apple.0 || y as u32 != apple.1 {
                segments.pop_back().unwrap();
            } else {
                apple = (rng.random_range(0..play_area.0), rng.random_range(0..play_area.1));
            }
            if x >= 0 && x < play_area.0 as i32 && y >= 0 && y < play_area.1 as i32 && !segments.contains(&(x as u32, y as u32)) {
                segments.push_front((x as u32, y as u32));
            } else {
                segments.clear();
                segments.push_front((play_area.0 / 2, play_area.1 / 2));
                direction = (0,0);
                apple = (rng.random_range(0..play_area.0), rng.random_range(0..play_area.1));
            }
        }

        for y in 0..window_size.1 {
            for x in 0..window_size.0 {
                let r = segments.contains(&(x*3,y)) || (apple.0 == x*3 && apple.1 == y);
                let g = segments.contains(&(x*3+1,y)) || (apple.0 == x*3+1 && apple.1 == y);
                let b = segments.contains(&(x*3+2,y)) || (apple.0 == x*3+2 && apple.1 == y);
                if r || g || b {
                    canvas.set_draw_color(Color::RGB(if r { 255 } else { 0 }, if g { 255 } else { 0 }, if b { 255 } else { 0 }));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, FRAME_DELAY));
    }
}
