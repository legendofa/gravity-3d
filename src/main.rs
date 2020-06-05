mod datatypes;

fn main() {
	let configurations = vec![
		vec![
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				10_f64.powi(13),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(100.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(-0.4_f64, 2.4_f64, 0.0_f64),
				10_f64.powi(11),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(-100.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(0.4_f64, -2.4_f64, 0.0_f64),
				10_f64.powi(2),
			),
		],
		vec![
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				10_f64.powi(13),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(100.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(-0.4_f64, 2.4_f64, 0.0_f64),
				10_f64.powi(5),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(-100.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(0.4_f64, -2.4_f64, 0.0_f64),
				10_f64.powi(2),
			),
		],
		vec![
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(0.0_f64, 0.0_f64, 0.0_f64),
				10_f64.powi(13),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(100.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(-0.4_f64, 2.4_f64, 0.0_f64),
				10_f64.powi(5),
			),
			datatypes::Body::new(
				cgmath::Vector3::<f64>::new(-200.0_f64, 0.0_f64, 0.0_f64),
				cgmath::Vector3::<f64>::new(1.0_f64, -0.5_f64, 0.0_f64),
				10_f64.powi(5),
			),
		],
	];

	let mut system = datatypes::System::new(configurations[2].clone());
	system.plot(1);
}
