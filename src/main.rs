mod stats; // stats.rs provides functionality for calculating mean, standard deviation, etc.
mod reader; // reader.rs provides functionality for parsing the csv data.

const CLASS_INDEX: usize = 30; // The index of the classification label.
const K: usize = 9;

const TP: usize = 0; // Actual =  1, Predicted =  1
const FN: usize = 1; // Actual =  1, Predicted = -1
const FP: usize = 2; // Actual = -1, Predicted =  1
const TN: usize = 3; // Actual = -1, Predicted = -1

fn main() {
    // Read the data into a usable form.
    let mut records = reader::read_records_from("data/wdbc.data.mb.csv");

    // Normalize the records, column by column.
    for i in 0..CLASS_INDEX {
	// Get the ith column.
	let column_i = records.iter()
	    .map(|x| x[i])
	    .collect::<Vec<f32>>();

	// Calculate statistics.
	let mean = stats::mean(&column_i).unwrap();
	let variance = stats::variance(&column_i, mean).unwrap();
	let standard_deviation = stats::standard_deviation(variance);

	// Produce normalized results and map them to the records.
	let normalized = normalize(&column_i, mean, standard_deviation).unwrap();
	for (n, record) in records.iter_mut().enumerate() {
	    record[i] = normalized[n];
	}
    }

    // Parition the records into a training set and testing set.
    let records_num = records.len();
    let train_percent = 0.6;
    let train_num = (train_percent * records_num as f32) as usize;
    let mut training_set = records[0..train_num].to_vec();
    let test_set = records[train_num..records_num].to_vec();

    // Classify the test set using k-nearest neighbors.
    let classifications = knn(K, &mut training_set, &test_set, l1_distance);

    // Build the confusion matrix.
    let mut cm: [f32; 4] = [0.0; 4];
    for (i, record) in test_set.iter().enumerate() {
	let actual = record[CLASS_INDEX] as i32;
	let predicted = classifications[i];

	if actual == 1 && predicted == 1 {cm[TP] += 1.0;}
	else if actual == 1 && predicted == -1 {cm[FN] += 1.0;}
	else if actual == -1 && predicted == 1 {cm[FP] += 1.0;}
	else if actual == -1 && predicted == -1 {cm[TN] += 1.0;}
    }

    // Calculate the accuracy.
    let accuracy = (cm[TP] + cm[TN]) / (cm[TP] + cm[FN] + cm[FP] + cm[FN]);

    // Print log.
    print_log(K, &cm, accuracy);
}

// Uses z-score normalization to normalize a set of numbers.
fn normalize(numbers: &Vec<f32>, mean: f32, standard_deviation: f32) -> Result<Vec<f32>, &'static str> {
    let result = numbers.iter()
	.map(|x| stats::zscore(*x, mean, standard_deviation))
	.collect::<Result<Vec<f32>, &'static str>>();
    result
}

// Calculates L1 distance.
fn l1_distance(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0.0;
    for (i, field) in a.iter().enumerate() {
	sum += f32::abs(field - b[i]);
    }
    sum
}

// Calculates L2 distance.
fn l2_distance(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0.0;
    for (i, field) in a.iter().enumerate() {
	sum += (field - b[i]) * (field - b[i]);
    }
    f32::sqrt(sum)
}

// Uses k-nearest neighbors to classify test data based on k, a provided test set, and a distance function.
fn knn<F>(k: usize, training_set: &mut Vec<Vec<f32>>, test_set: &Vec<Vec<f32>>, distance: F) -> Vec<i32> where
    F: Fn(&[f32], &[f32]) -> f32 {
    let mut classifications: Vec<i32> = Vec::new();

    // Classify each of the data in the test set.
    for unclassified in test_set.iter() {
	// Sort the training data based on its distance to the unclassified point.
	training_set
	    .sort_by(|a, b| {
		let distance_a = distance(&unclassified[0..CLASS_INDEX], &a[0..CLASS_INDEX]);
		let distance_b = distance(&unclassified[0..CLASS_INDEX], &b[0..CLASS_INDEX]);
		distance_b.partial_cmp(&distance_a).unwrap()
	    });

	// Determine the majority classification for the k-nearest neighbors.
	let vote = training_set.iter()
	    .take(k) // Only look at k-nearest neighbors.
	    .fold(0.0, |acc, x| acc + x[CLASS_INDEX]); // Sum the labels of the k-nearest neighbors.

	// If the vote is negative more neighbors were benign.
	// Potential false positive is given if k is even and there is no plurality.
	if vote < 0.0 {
	    classifications.push(-1);
	} else {
	    classifications.push(1) 
	}
    }
    classifications
}

// Prints some information.
fn print_log(k: usize, cm: &[f32], accuracy: f32) {
    let log = format!("k: {}\nTP: {}, FN: {}, FP: {}, TN: {}\naccuracy: {}", k, cm[TP], cm[FN], cm[FP], cm[TN], accuracy);
    println!("{}", log);
}
