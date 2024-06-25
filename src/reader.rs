const FIELD_COUNT: usize = 31;

pub fn read_records_from(path: &str) -> Vec<Vec<f32>> {
    let mut reader = csv::ReaderBuilder::new()
	.from_path(path)
	.unwrap();

    reader.records()
	.map(|result| {
	    let sr = result.unwrap();
	    let columns: Vec<f32> = sr.iter()
		.take(FIELD_COUNT)
		.map(|x| x.parse::<f32>().unwrap())
		.collect();
	    columns
	})
	.collect::<Vec<Vec<f32>>>()
}
