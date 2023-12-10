#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sequence(Vec<i64>);

impl Sequence {
    pub fn new(numbers: Vec<i64>) -> Self {
        if numbers.len() == 0 {
            panic!("Empty numbers array");
        }

        Self(numbers)
    }

    pub fn is_zeros(&self) -> bool {
        self.0.iter().all(|num| *num == 0i64)
    }

    pub fn get_difference_sequence(&self) -> Self {
        let next_length = self.0.len() - 1;
        let mut numbers = Vec::with_capacity(next_length);

        for index in 0..next_length {
            numbers.push(self.0[index + 1] - self.0[index]);
        }

        Self(numbers)
    }

    pub fn last(&self) -> &i64 {
        self.0.last().unwrap()
    }

    pub fn first(&self) -> &i64 {
        self.0.first().unwrap()
    }
}
