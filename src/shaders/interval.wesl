struct Interval {
	min: f32,
	max: f32,
}
fn new_interval(min: f32, max: f32) -> Interval {
	var interval: Interval;
	interval.min = min;
	interval.max = max;
	return interval;
}