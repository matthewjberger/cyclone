use impulse::{Particle, Real};
use macroquad::prelude::*;

const PARTICLE_TIMEOUT_SECS: f32 = 5.0;
const AMMO_COUNT: usize = 10;
const CAMERA_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.1;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shot {
	Pistol,
	Artillery,
	Fireball,
	Laser,
	Grenade,
}

struct Round {
	particle: Particle,
	start_time: Option<f32>,
	alive: bool,
	trajectory: Vec<Vec3>, // Store the positions that the particle has traveled through
}

struct GameState {
	rounds: Vec<Round>,
	next_shot: Shot,
	should_fire: bool,
	camera: Camera3D,
	yaw: f32,
	pitch: f32,
}

impl GameState {
	fn new() -> Self {
		let rounds = (0..AMMO_COUNT)
			.map(|_| Round {
				particle: shot_as_particle(Shot::Pistol, impulse::Vector3::zero()),
				start_time: None,
				alive: false,
				trajectory: Vec::new(), // Initialize the trajectory
			})
			.collect();

		Self {
			rounds,
			next_shot: Shot::Pistol,
			should_fire: false,
			camera: Camera3D {
				position: vec3(-30.0, 5.0, 10.0),
				up: vec3(0.0, 1.0, 0.0),
				target: vec3(0.0, 0.0, 0.0),
				..Default::default()
			},
			yaw: 0.0,
			pitch: 0.0,
		}
	}
}

#[macroquad::main("Ballistics Demo")]
async fn main() {
	let mut game_state = GameState::new();

	loop {
		clear_background(LIGHTGRAY);

		set_camera(&game_state.camera);

		handle_input(&mut game_state);
		update_physics(&mut game_state);
		render_scene(&game_state);

		set_default_camera();
		render_ui(&game_state);

		next_frame().await
	}
}

fn handle_input(game_state: &mut GameState) {
	if is_key_pressed(KeyCode::Key1) {
		game_state.next_shot = Shot::Pistol;
	}
	if is_key_pressed(KeyCode::Key2) {
		game_state.next_shot = Shot::Artillery;
	}
	if is_key_pressed(KeyCode::Key3) {
		game_state.next_shot = Shot::Fireball;
	}
	if is_key_pressed(KeyCode::Key4) {
		game_state.next_shot = Shot::Laser;
	}
	if is_key_pressed(KeyCode::Key5) {
		game_state.next_shot = Shot::Grenade;
	}
	if is_key_pressed(KeyCode::Space) {
		game_state.should_fire = true;
	}

	// Camera rotation
	let mouse_delta = mouse_delta_position();
	let (mouse_dx, mouse_dy) = (mouse_delta.x, mouse_delta.y);
	game_state.yaw -= mouse_dx * MOUSE_SENSITIVITY;
	game_state.pitch -= mouse_dy * MOUSE_SENSITIVITY;
	game_state.pitch = game_state.pitch.clamp(-89.0, 89.0);

	let (yaw_sin, yaw_cos) = game_state.yaw.to_radians().sin_cos();
	let (pitch_sin, pitch_cos) = game_state.pitch.to_radians().sin_cos();
	let forward = vec3(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();
	let right = vec3(-yaw_sin, 0.0, yaw_cos).normalize();
	let up = right.cross(forward);

	// Camera movement
	let mut movement = Vec3::ZERO;
	if is_key_down(KeyCode::W) {
		movement += forward;
	}
	if is_key_down(KeyCode::S) {
		movement -= forward;
	}
	if is_key_down(KeyCode::A) {
		movement -= right;
	}
	if is_key_down(KeyCode::D) {
		movement += right;
	}
	if is_key_down(KeyCode::Q) {
		movement -= up;
	}
	if is_key_down(KeyCode::E) {
		movement += up;
	}

	game_state.camera.position += movement.normalize_or_zero() * CAMERA_SPEED * get_frame_time();
	game_state.camera.target = game_state.camera.position + forward;
	game_state.camera.up = up;
}

fn update_physics(game_state: &mut GameState) {
	let dt = get_frame_time();

	for round in &mut game_state.rounds {
		if round.alive {
			round.particle.integrate(dt);
			round.trajectory.push(to_vec3(&round.particle.position)); // Store the position in the trajectory
		} else if game_state.should_fire {
			round.start_time = Some(get_time() as f32);
			round.alive = true;
			round.trajectory.clear(); // Clear previous trajectory
			let spawn_pos = impulse::Vector3::new(0.0, 1.5, 0.0); // Fixed spawn point at the center
			round.particle = shot_as_particle(game_state.next_shot, spawn_pos);
			round.trajectory.push(to_vec3(&spawn_pos)); // Start trajectory from the origin
											// Adjust velocity based on the shot type, but keep it pointing along positive Z
			round.particle.velocity =
				impulse::Vector3::new(0.0, round.particle.velocity.y(), round.particle.velocity.z());
			game_state.should_fire = false;
			break;
		}

		if round.alive {
			let out_of_bounds = round.particle.position.y() < -5.0 || round.particle.position.z() > 200.0;
			let expired = match round.start_time {
				Some(start_time) => (get_time() as f32 - start_time) > PARTICLE_TIMEOUT_SECS,
				None => true,
			};
			if out_of_bounds || expired {
				round.start_time = None;
				round.alive = false;
			}
		}
	}
}

fn render_scene(game_state: &GameState) {
	// Draw ground grid
	for i in (0..=20).step_by(1) {
		let pos = i as f32 * 10.0 - 100.0;
		draw_line_3d(vec3(-100.0, 0.0, pos), vec3(100.0, 0.0, pos), BLACK);
		draw_line_3d(vec3(pos, 0.0, -100.0), vec3(pos, 0.0, 100.0), BLACK);
	}

	// Draw launch point
	draw_sphere(Vec3::new(0.0, 1.5, 0.0), 0.5, None, GREEN);

	// Draw particles and their trajectories
	for round in &game_state.rounds {
		if round.alive {
			draw_sphere(to_vec3(&round.particle.position), 0.5, None, BLUE);

			// Draw trajectory
			if round.trajectory.len() > 1 {
				for i in 0..(round.trajectory.len() - 1) {
					draw_line_3d(round.trajectory[i], round.trajectory[i + 1], GREEN);
				}
			}
		}
	}
}

fn render_ui(game_state: &GameState) {
	let text = format!("Current Ammo Type: {:?}", game_state.next_shot);
	draw_text(&text, 10.0, 30.0, 30.0, DARKGRAY);
	draw_text("WASD: Move, Mouse: Look, Q/E: Up/Down", 10.0, 60.0, 20.0, DARKGRAY);
	draw_text("1-5: Change ammo, Space: Fire", 10.0, 90.0, 20.0, DARKGRAY);
}

fn shot_as_particle(shot: Shot, position: impulse::Vector3) -> Particle {
	match shot {
		Shot::Pistol => {
			Particle {
				inverse_mass: (2.0 as Real).recip(),             // 2.0 kg
				velocity: impulse::Vector3::new(0.0, 0.0, 35.0), // 35 m/s
				acceleration: impulse::Vector3::new(0.0, -1.0, 0.0),
				damping: 0.99,
				position,
				force_accumulator: impulse::Vector3::zero(),
			}
		},
		Shot::Artillery => {
			Particle {
				inverse_mass: (200.0 as Real).recip(),            // 200.0 kg
				velocity: impulse::Vector3::new(0.0, 30.0, 40.0), // 50 m/s
				acceleration: impulse::Vector3::new(0.0, -20.0, 0.0),
				damping: 0.99,
				position,
				force_accumulator: impulse::Vector3::zero(),
			}
		},
		Shot::Fireball => {
			Particle {
				inverse_mass: (1.0 as Real).recip(),                // 1.0 kg
				velocity: impulse::Vector3::new(0.0, 00.0, 10.0),   // 5 m/s
				acceleration: impulse::Vector3::new(0.0, 0.6, 0.0), // Floats up
				damping: 0.9,
				position,
				force_accumulator: impulse::Vector3::zero(),
			}
		},
		Shot::Laser => {
			// Note that this is the kind of laser bolt seen in films, not a realistic laser beam!
			Particle {
				inverse_mass: (0.1 as Real).recip(),                // 1.0 kg
				velocity: impulse::Vector3::new(0.0, 0.0, 100.0),   // 100 m/s
				acceleration: impulse::Vector3::new(0.0, 0.0, 0.0), // No gravity
				damping: 0.99,
				position,
				force_accumulator: impulse::Vector3::zero(),
			}
		},
		Shot::Grenade => {
			Particle {
				inverse_mass: (0.9 as Real).recip(), // 200.0 kg
				velocity: impulse::Vector3::new(0.0, 15.0, 10.0),
				acceleration: impulse::Vector3::new(0.0, -10.0, 0.0),
				damping: 0.99,
				position,
				force_accumulator: impulse::Vector3::zero(),
			}
		},
	}
}

fn to_vec3(vec: &impulse::Vector3) -> Vec3 {
	vec3(vec.x(), vec.y(), vec.z())
}
