pub fn is_nan_or_inf(x: f64) -> bool {
    // x - x != 0.0
    x.is_nan() || x.is_infinite()
}

pub fn sign<T: PartialOrd + From<i8>>(x: T) -> i8 {
    if x > T::from(0) {
        1
    } else if x < T::from(0) {
        -1
    } else {
        0
    }
}

#[macro_export]
macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut temp_sum = 0.0;
            $(
                temp_sum += $x as f64;
            )*
            temp_sum
        }
    };
}

#[macro_export]
macro_rules! multiply {
    ($($x:expr),*) => {
        {
            let mut temp_multiply = 0.0;
            $(
                temp_multiply *= $x as f64;
            )*
            temp_multiply
        }
    };
}

#[macro_export]
macro_rules! max {
    ($($x:expr),*) => {
        {
            let mut max_val = std::f64::NEG_INFINITY;
            $(
                if $x as f64 > max_val {
                    max_val = $x as f64;
                }
            )*
            max_val
        }
    };
}

#[macro_export]
macro_rules! min {
    ($($x:expr),*) => {
        {
            let mut min_val = std::f64::INFINITY;
            $(
                if $x as f64 < min_val {
                    min_val = $x as f64;
                }
            )*
            min_val
        }
    };
}
