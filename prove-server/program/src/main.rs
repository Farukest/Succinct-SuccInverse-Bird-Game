#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Serialize, Deserialize};

const SCRN_WIDTH: f32 = 432.0;
const PIPE_WIDTH: f32 = 52.0;
const BIRD_SPRITE_WIDTH: f32 = 50.0;
const BIRD_SPRITE_HEIGHT: f32 = 50.0;
const RAD: f32 = std::f32::consts::PI / 180.0;

const GRAVITY: f32 = 0.125;
const THRUST: f32 = 3.6;
const PIPE_GAP: f32 = 150.0;
const BIRD_X: f32 = 320.0;
const BASE_SPEED: f32 = 2.0;
const SPEED_INCREMENT: f32 = 0.2;

#[derive(Clone, Copy, Debug)]
struct Pipe {
    x: f32,
    y: f32,
    gap: f32,
}

#[derive(Clone, Copy, Debug)]
struct Bird {
    x: f32,
    y: f32,
    speed: f32,
    rotation: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Play,
    GameOver,
}

// Commit edilecek çıktı struct’ı
#[derive(Serialize, Deserialize, Debug)]
struct GameOutput {
    score: u32,
    username: String,
}


fn main() {
    let flaps: Vec<u32> = sp1_zkvm::io::read::<Vec<u32>>();
	// let flaps: Vec<u32> = vec![42, 49, 57, 63, 70, 76, 82, 148, 215, 222, 229, 237, 315, 322, 329, 334, 342, 415, 474, 540, 603, 610, 617, 623, 630, 697, 704, 712, 719, 729, 808, 875, 882, 942, 949, 956, 962, 969, 1039, 1107, 1115, 1123, 1130];
    let username: String = sp1_zkvm::io::read();
	let mut score_flaps: Vec<u32> = Vec::new();
    let mut bird = Bird {
        x: BIRD_X,
        y: 260.0,
        speed: 0.0,
        rotation: 0.0
    };
    let mut pipes: Vec<Pipe> = Vec::new();
    let mut score: u32 = 0;
    let mut frame_count: u32 = 0;
    let mut game_state = GameState::Play;
    let mut dx = BASE_SPEED;
	let mut next_spawn_frame: u32 = 0; // The frame at which the next pipe will spawn
    const BASE_PIPE_DISTANCE: f32 = 200.0; // Fixed distance (100 frames * 2 speed for dx = 2.0)
    let mut flap_index: usize = 0;
    let mut moved: bool = true;

	bird.rotation = 0.0;
	bird.y += if frame_count % 10 == 0 { (frame_count as f32 * RAD).sin() } else { 0.0 };
    while game_state != GameState::GameOver {
        match game_state {
            GameState::Play => {
				
                bird.y += bird.speed;
                set_rotation(&mut bird);
                bird.speed += GRAVITY;
				
				
				if collision_and_score(&bird, &mut pipes, &mut score, frame_count, &mut moved, &mut dx, &mut score_flaps) {
                    game_state = GameState::GameOver;
                }
				
                update_pipes(&mut pipes, frame_count, dx, &mut moved, &mut next_spawn_frame);
				
				
				frame_count += 1;
				
				if flap_index < flaps.len() && frame_count == flaps[flap_index] {
					flap_index += 1;
                    flap(&mut bird);
                }
				
			
            }
            GameState::GameOver => {
				break;
            }
        }
					
    }
	
	println!("SCOREEEEEEEEEEE: {}", score);
	// sp1_zkvm::io::commit(&score);
	
	let output = GameOutput {
        score,
        username,
    };
    sp1_zkvm::io::commit::<GameOutput>(&output);
	
}

fn collision_and_score(bird: &Bird, pipes: &mut Vec<Pipe>, score: &mut u32, frame_count: u32, moved: &mut bool, dx: &mut f32, score_flaps: &mut Vec<u32>) -> bool {
    if pipes.is_empty() {
        return false;
    }

    let pipe = &mut pipes[0];
    let roof = pipe.y + 320.0;
    let floor = roof + pipe.gap; // Dinamik gap
    let w = PIPE_WIDTH;

	const TOLERANCE: f32 = 5.0;
    if 
        bird.x + BIRD_SPRITE_WIDTH / 2.0 >= pipe.x + TOLERANCE && // Tolerance when entering the pipe
        bird.x - BIRD_SPRITE_WIDTH / 2.0 <= pipe.x + w - TOLERANCE // Tolerance when exiting the pipe
    {
        if 
            bird.y - BIRD_SPRITE_HEIGHT / 2.0 <= roof - TOLERANCE || // Tolerance for the top pipe
            bird.y + BIRD_SPRITE_HEIGHT / 2.0 >= floor + TOLERANCE    // Tolerance for the bottom pipe
        {
					
            return true;
        } else if *moved && bird.x < pipe.x {
            *score += 1;
            score_flaps.push(frame_count);
            *moved = false;
            if *score % 5 == 0 {
                *dx += *dx * SPEED_INCREMENT; // Speed increases by 20% every 5 scores
            }
        }
    }
    false
}



fn set_rotation(bird: &mut Bird) {
    if bird.speed <= 0.0 {
        bird.rotation = (-25.0_f32).max((-25.0 * bird.speed) / (-1.0 * THRUST));
    } else if bird.speed > 0.0 {
        bird.rotation = 90.0_f32.min((90.0 * bird.speed) / (THRUST * 2.0));
    }
}

fn flap(bird: &mut Bird) {
    if bird.y > 0.0 {
        bird.speed = -THRUST;
    }
}

fn update_pipes(pipes: &mut Vec<Pipe>, frame_count: u32, dx: f32, moved: &mut bool, next_spawn_frame: &mut u32) {
    const BASE_PIPE_DISTANCE: f32 = 200.0;
    if frame_count >= *next_spawn_frame {
        
        let pipe_y = -160.0 + 50.0 * (frame_count as f32 * 0.05).sin(); // Top pipe y position, oscillates between -210 and -110
        let gap = 150.0; // Sabit gap
        
        pipes.push(Pipe {
            x: -PIPE_WIDTH,
            y: pipe_y, // Dynamic top pipe position
            gap: gap,  // Fixed gap
        });
        *next_spawn_frame = frame_count + (BASE_PIPE_DISTANCE / dx).floor() as u32;

    }
    for pipe in pipes.iter_mut() {
        pipe.x += dx;
    }
    if let Some(first_pipe) = pipes.first() {
        if first_pipe.x > SCRN_WIDTH {
            pipes.remove(0);
            *moved = true;
        }
    }
}
