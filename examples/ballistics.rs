use impulse::{Particle, Real};
use kiss3d::{
	event::{Action, Key, WindowEvent},
	light::Light,
	text::Font,
	window::Window,
};
use na::{Point2, Point3, Translation3};
use nalgebra as na;
use std::time::Instant;

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
enum Shot {
	#[default]
	Unassigned,
	Pistol,
	Artillery,
	Fireball,
	Laser,
	Grenade,
}

#[derive(Default, Copy, Clone)]
struct Round {
	pub particle: Particle,
	pub kind: Shot,
	pub start_time: Option<Instant>,
}

#[derive(Default)]
struct Gun {
	pub rounds: [Round; Self::AMMO_COUNT],
	pub next_shot_kind: Shot,
}

impl Gun {
	pub const AMMO_COUNT: usize = 16;
	pub const PARTICLE_TIMEOUT_SECS: usize = 5;

	pub fn fire(&mut self) {
		if let Some(available_round) = self.rounds.iter_mut().find(|round| round.kind == Shot::Unassigned) {
			match self.next_shot_kind {
				Shot::Pistol => {
					available_round.particle.inverse_mass = (2.0 as Real).recip(); // 2.0 kg
					available_round.particle.velocity = impulse::Vector3::new(0.0, 0.0, 35.0); // 35 m/s
					available_round.particle.acceleration = impulse::Vector3::new(0.0, -1.0, 0.0);
					available_round.particle.damping = 0.99;
				},
				Shot::Artillery => {
					available_round.particle.inverse_mass = (200.0 as Real).recip(); // 200.0 kg
					available_round.particle.velocity = impulse::Vector3::new(0.0, 30.0, 40.0); // 50 m/s
					available_round.particle.acceleration = impulse::Vector3::new(0.0, -20.0, 0.0);
					available_round.particle.damping = 0.99;
				},
				Shot::Fireball => {
					available_round.particle.inverse_mass = (1.0 as Real).recip(); // 1.0 kg - mostly blast damage
					available_round.particle.velocity = impulse::Vector3::new(0.0, 0.0, 10.0); // 5 m/s
					available_round.particle.acceleration = impulse::Vector3::new(0.0, 0.6, 0.0); // Floats up
					available_round.particle.damping = 0.9;
				},
				Shot::Laser => {
					// Note that this is the kind of laser bolt seen in films,
					// not a realistic laser beam!
					available_round.particle.inverse_mass = (0.1 as Real).recip(); // 1.0 kg - mostly blast damage
					available_round.particle.velocity = impulse::Vector3::new(0.0, 0.0, 100.0); // 100 m/s
					available_round.particle.acceleration = impulse::Vector3::new(0.0, 0.0, 0.0); // No gravity
					available_round.particle.damping = 0.99;
				},
				Shot::Grenade => {
					available_round.particle.inverse_mass = (0.9 as Real).recip(); // 200.0 kg
					available_round.particle.velocity = impulse::Vector3::new(0.0, 15.0, 10.0);
					available_round.particle.acceleration = impulse::Vector3::new(0.0, -10.0, 0.0);
					available_round.particle.damping = 0.99;
				},
				Shot::Unassigned => {},
			}
			available_round.particle.position = impulse::Vector3::new(0.0, 1.5, 0.0);
			available_round.start_time = Some(Instant::now());
			available_round.kind = self.next_shot_kind;
			available_round.particle.force_accumulator = impulse::Vector3::zero();
		}
	}

	pub fn update(&mut self, last_frame_duration: Real) {
		for round in self.rounds.iter_mut() {
			if round.kind == Shot::Unassigned {
				continue;
			}

			round.particle.integrate(last_frame_duration);

			let out_of_bounds = round.particle.position.y() < 0.0 || round.particle.position.z() > 200.0;
			let expired = match round.start_time {
				Some(instant) => (Instant::now() - instant).as_secs() > Self::PARTICLE_TIMEOUT_SECS as _,
				None => true,
			};
			if out_of_bounds || expired {
				round.kind = Shot::Unassigned;
			}
		}
	}
}

fn main() {
	let mut window = Window::new("Impulse Physics Engine - Ballistics Demo");
	window.set_light(Light::StickToCamera);
	let font = Font::default();

	let mut bullets = Vec::new();
	for _ in 0..Gun::AMMO_COUNT {
		let mut bullet = window.add_sphere(0.5);
		bullet.set_visible(false);
		bullet.set_color(0.0, 1.0, 1.0);
		bullets.push(bullet);
	}

	let mut gun = Gun::default();
	gun.next_shot_kind = Shot::Pistol;

	while window.render() {
		for event in window.events().iter() {
			if let WindowEvent::Key(key, Action::Press, _) = event.value {
				match key {
					Key::Space => gun.fire(),
					Key::Key1 => gun.next_shot_kind = Shot::Pistol,
					Key::Key2 => gun.next_shot_kind = Shot::Artillery,
					Key::Key3 => gun.next_shot_kind = Shot::Fireball,
					Key::Key4 => gun.next_shot_kind = Shot::Laser,
					Key::Key5 => gun.next_shot_kind = Shot::Grenade,
					_ => {},
				}
			}
		}

		// Fake the last frame's duration
		let last_frame_duration = 0.01;
		gun.update(last_frame_duration);

		window.draw_text(
			&format!("Current Ammo Type: {:?}", gun.next_shot_kind),
			&Point2::origin(),
			36.0,
			&font,
			&Point3::new(0.0, 1.0, 1.0),
		);

		for offset in (0..200).step_by(10) {
			window.draw_line(
				&Point3::new(-5.0, 0.0, offset as _),
				&Point3::new(5.0, 0.0, offset as _),
				&Point3::new(0.75, 0.75, 0.75),
			);
		}

		for (round, bullet) in gun.rounds.iter().zip(bullets.iter_mut()) {
			let is_used = round.kind != Shot::Unassigned;
			bullet.set_visible(is_used);
			if !is_used {
				continue;
			}

			bullet.set_local_translation(Translation3::new(
				round.particle.position.x() as _,
				round.particle.position.y() as _,
				round.particle.position.z() as _,
			));
		}
	}
}
