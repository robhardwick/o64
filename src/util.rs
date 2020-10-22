#[macro_export]
macro_rules! bool2f32 {
    ($e:expr) => {
        $e as u8 as f32
    };
}

#[macro_export]
macro_rules! mutate {
    ($rng:expr, $value:expr, $min:expr, $max:expr) => {
        match $value * $rng.gen_range(0.99, 1.01) {
            v if v < $min => $min,
            v if v > $max => $max,
            v => v,
        }
    };
}
