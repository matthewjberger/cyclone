use impulse::{Particle, Vector3};
use macroquad::prelude::*;
use rand::gen_range;

const FIREWORK_COUNT: usize = 5;
const PARTICLE_COUNT: usize = 100;
const LAUNCH_DELAY: f32 = 2.0;
const EXPLOSION_DURATION: f32 = 1.5;
const SCREEN_BOUNDS: (f32, f32, f32) = (30.0, 40.0, 30.0); // (width, height, depth)

#[derive(Clone, Copy)]
enum FireworkType {
	Standard,
	Sparkler,
	Willow,
	Chrysanthemum,
	MultiColor,
	Kamuro,
}

struct ExplosionStage {
	particles: Vec<Particle>,
	color: Color,
	start_time: f32,
	duration: f32,
}

struct Firework {
	rocket: Particle,
	stages: Vec<ExplosionStage>,
	current_stage: usize,
	firework_type: FireworkType,
	exploded: bool,
}

impl Firework {
	fn new() -> Self {
		let x = gen_range(-SCREEN_BOUNDS.0 / 2.0, SCREEN_BOUNDS.0 / 2.0);
		let y = 0.0;
		let z = gen_range(-SCREEN_BOUNDS.2 / 2.0, SCREEN_BOUNDS.2 / 2.0);

		let rocket = Particle {
			position: Vector3::new(x, y, z),
			velocity: Vector3::new(0.0, gen_range(20.0, 25.0), 0.0),
			acceleration: Vector3::new(0.0, -9.8, 0.0),
			damping: 0.99,
			inverse_mass: 1.0,
			force_accumulator: Vector3::zero(),
		};

		let firework_type = match gen_range(0, 7) {
			0 => FireworkType::Standard,
			1 => FireworkType::Sparkler,
			2 => FireworkType::Willow,
			3 => FireworkType::Chrysanthemum,
			4 => FireworkType::MultiColor,
			_ => FireworkType::Kamuro,
		};

		Firework {
			rocket,
			stages: Vec::new(),
			current_stage: 0,
			firework_type,
			exploded: false,
		}
	}

	fn update(&mut self, dt: f32, current_time: f32) {
		if !self.exploded {
			self.rocket.integrate(dt);
			if self.rocket.velocity.y() <= 0.0 || self.rocket.position.y() > SCREEN_BOUNDS.1 {
				self.explode(current_time);
			}
		} else if self.current_stage < self.stages.len() {
			let stage = &mut self.stages[self.current_stage];
			for particle in &mut stage.particles {
				particle.integrate(dt);
			}
			if current_time - stage.start_time > stage.duration {
				self.current_stage += 1;
				if self.current_stage < self.stages.len() {
					self.stages[self.current_stage].start_time = current_time;
				}
			}
		}
	}

	fn explode(&mut self, current_time: f32) {
		self.stages = match self.firework_type {
			FireworkType::Standard => vec![self.create_explosion(self.random_color(), EXPLOSION_DURATION, 1.0)],
			FireworkType::Sparkler => vec![self.create_sparkler_explosion()],
			FireworkType::Willow => vec![self.create_willow_explosion()],
			FireworkType::Chrysanthemum => vec![self.create_chrysanthemum_explosion()],
			FireworkType::MultiColor => self.create_multicolor_explosion(),
			FireworkType::Kamuro => self.create_kamuro_explosion(),
		};
		self.stages[0].start_time = current_time;
		self.exploded = true;
	}

	fn create_explosion(&self, color: Color, duration: f32, speed_factor: f32) -> ExplosionStage {
		let particles = (0..PARTICLE_COUNT)
			.map(|_| {
				let velocity = Vector3::new(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0), gen_range(-1.0, 1.0))
					.normalize() * gen_range(5.0, 10.0)
					* speed_factor;

				Particle {
					position: self.rocket.position,
					velocity,
					acceleration: Vector3::new(0.0, -2.0, 0.0),
					damping: 0.99,
					inverse_mass: 1.0,
					force_accumulator: Vector3::zero(),
				}
			})
			.collect();

		ExplosionStage {
			particles,
			color,
			start_time: 0.0,
			duration,
		}
	}

	fn create_sparkler_explosion(&self) -> ExplosionStage {
		let mut stage = self.create_explosion(self.random_color(), EXPLOSION_DURATION, 0.8);
		for particle in &mut stage.particles {
			particle.damping = 0.95;
		}
		stage
	}

	fn create_willow_explosion(&self) -> ExplosionStage {
		self.create_explosion(self.random_color(), EXPLOSION_DURATION * 1.5, 0.6)
	}

	fn create_chrysanthemum_explosion(&self) -> ExplosionStage {
		self.create_explosion(self.random_color(), EXPLOSION_DURATION * 1.2, 1.2)
	}

	fn create_multicolor_explosion(&self) -> Vec<ExplosionStage> {
		let colors = [RED, GREEN, BLUE, YELLOW, PURPLE];
		(0..3)
			.map(|i| self.create_explosion(colors[i % colors.len()], EXPLOSION_DURATION, 0.8 + 0.2 * i as f32))
			.collect()
	}

	fn create_kamuro_explosion(&self) -> Vec<ExplosionStage> {
		let color = self.random_color();
		vec![
			self.create_explosion(color, EXPLOSION_DURATION * 0.5, 1.2),
			self.create_willow_explosion(),
		]
	}

	fn random_color(&self) -> Color {
		Color::new(gen_range(0.5, 1.0), gen_range(0.5, 1.0), gen_range(0.5, 1.0), 1.0)
	}

	fn draw(&self) {
		if !self.exploded {
			draw_sphere(
				vec3(
					self.rocket.position.x() as f32,
					self.rocket.position.y() as f32,
					self.rocket.position.z() as f32,
				),
				0.5,
				None,
				WHITE,
			);
		} else if self.current_stage < self.stages.len() {
			let stage = &self.stages[self.current_stage];
			let fade = 1.0 - ((get_time() as f32 - stage.start_time) / stage.duration);
			for (i, particle) in stage.particles.iter().enumerate() {
				let particle_color = match self.firework_type {
					FireworkType::Sparkler => {
						let sparkle = if i % 2 == 0 { 1.0 } else { 0.5 };
						Color::new(stage.color.r, stage.color.g, stage.color.b, fade * sparkle)
					},
					_ => Color::new(stage.color.r, stage.color.g, stage.color.b, fade),
				};

				draw_sphere(
					vec3(
						particle.position.x() as f32,
						particle.position.y() as f32,
						particle.position.z() as f32,
					),
					0.2,
					None,
					particle_color,
				);
			}
		}
	}
}

struct FireworksDisplay {
	fireworks: Vec<Firework>,
	last_launch_time: f32,
	camera: Camera3D,
}

impl FireworksDisplay {
	fn new() -> Self {
		FireworksDisplay {
			fireworks: Vec::new(),
			last_launch_time: 0.0,
			camera: Camera3D {
				position: vec3(0.0, 20.0, 50.0),
				up: vec3(0.0, 1.0, 0.0),
				target: vec3(0.0, 20.0, 0.0),
				..Default::default()
			},
		}
	}

	fn update(&mut self, dt: f32) {
		let current_time = get_time() as f32;
		if current_time - self.last_launch_time > LAUNCH_DELAY && self.fireworks.len() < FIREWORK_COUNT {
			self.fireworks.push(Firework::new());
			self.last_launch_time = current_time;
		}

		for firework in &mut self.fireworks {
			firework.update(dt, current_time);
		}

		self.fireworks
			.retain(|f| !f.exploded || f.current_stage < f.stages.len());
	}

	fn draw(&self) {
		clear_background(BLACK);
		set_camera(&self.camera);

		for firework in &self.fireworks {
			firework.draw();
		}

		set_default_camera();
		draw_text("Fireworks Display", 10.0, 30.0, 30.0, WHITE);
	}
}

#[macroquad::main("Fireworks Display")]
async fn main() {
	let mut display = FireworksDisplay::new();

	loop {
		display.update(get_frame_time());
		display.draw();
		next_frame().await
	}
}
