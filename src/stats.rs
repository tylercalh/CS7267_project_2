// Calculates the mean of a given set of numbers.
pub fn mean(numbers: &Vec<f32>) -> Result<f32, &'static str> {
    if numbers.len() == 0 {return Err("Divide by zero.");}
    let n = numbers.len() as f32;
    let sum = numbers.iter()
	.fold(0.0, |acc, x| acc + *x);
    let mean = sum / n;
    Ok(mean)
}

// Calculates the variance of a given set of numbers.
pub fn variance(numbers: &Vec<f32>, mean: f32) -> Result<f32, &'static str> {
    if numbers.len() == 0 {return Err("Divide by zero.");}
    let n = numbers.len() as f32;
    let sum = numbers.iter()
	.fold(0.0, |acc, x| acc + ((*x - mean) * (*x - mean)));
    let variance = sum / n;
    Ok(variance)
}

// Calculates standard deviation given variance.
pub fn standard_deviation(variance: f32) -> f32 {
    let standard_deviation = f32::sqrt(variance);
    standard_deviation
}

// Calculates a numbers z-score given a mean and standard deviation.
pub fn zscore(number: f32, mean: f32, standard_deviation: f32) -> Result<f32, &'static str> {
    if standard_deviation == 0.0 {return Err("Divide by zero.");}
    let normalized_number = (number - mean) / standard_deviation;
    Ok(normalized_number)
}
