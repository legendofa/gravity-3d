use cgmath::*;
use matplotrust::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as G;
use std::ops::*;

#[derive(Copy, Clone)]
pub struct Body {
	pub position: Vector3<f64>,
	pub velocity: Vector3<f64>,
	pub mass: f64,
}

impl Body {
	pub fn new(position: Vector3<f64>, velocity: Vector3<f64>, mass: f64) -> Body {
		Body {
			position,
			velocity,
			mass,
		}
	}

	pub fn calculate_force(&self, other: &Body, radius: f64) -> f64 {
		-(G * self.mass * other.mass) / radius.powi(2)
	}

	pub fn update_velocity(&mut self, other: Body) {
		let distance = self.position.sub(other.position);
		let force = self.calculate_force(&other, distance.magnitude());
		self.velocity = self.velocity.add(
			self.position
				.sub(other.position)
				.normalize()
				.mul(force)
				.div(self.mass),
		);
		//println!("{:#?}", self.velocity);
	}

	pub fn update_position(&mut self) {
		self.position = self.position.add(self.velocity);
	}

	pub fn print(&self) {
		println!(
			"X-position {}, Y-position {}, Mass {}	Velocity {}",
			self.position.x,
			self.position.y,
			self.mass,
			self.position.distance(self.velocity)
		)
	}
}

pub struct System {
	pub bodies: Vec<Body>,
}

impl System {
	pub fn new(bodies: Vec<Body>) -> System {
		System { bodies }
	}

	pub fn update(&mut self) {
		let secondary_bodies = self.bodies.clone();
		for primary_body in &mut self.bodies {			
			for secondary_body in &secondary_bodies {
				println!("{:#?}, {:#?}", primary_body.position, primary_body.velocity);
				primary_body.update_velocity(*secondary_body);
				//primary_body.update_position();
			}
		}
	}

	pub fn plot(&mut self, cycles: u32) {
		let mut cycle_values: Vec<Vec<Vector3<f64>>> = Vec::new();
		for _ in 0..cycles {
			cycle_values.push(Vec::new());
			for body in &mut self.bodies {
				cycle_values.last_mut().unwrap().push(body.position);
			}
			self.update();
		}
		let mut lp;
		let mut figure = Figure::new();
		for (i, _) in self.bodies.iter().enumerate() {
			let mut values: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
			for cycle in &mut cycle_values {
				values[0].push(cycle[i].x);
				values[1].push(cycle[i].z);
			}
			lp = line_plot::<f64, f64>(values[0].clone(), values[1].clone(), None);
			figure.add_plot(lp.clone());
			figure.add_plot(lp.clone());
		}
		print!("{:#?}", figure.save("./fig.png", None));
	}
}
