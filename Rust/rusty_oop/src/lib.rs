#[allow(dead_code)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> AveragedCollection {
        let total: i32 = list.iter().sum();
        let average = total as f64 / list.len() as f64;
        AveragedCollection { list, average }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averaged_collection_is_built_correctly() {
        let list = vec![1,2,3];
        let avg_col = AveragedCollection::new(list);
        assert_eq!(avg_col.average(), 2.0);
    }

    #[test]
    fn add_works_updates_the_avg() {
        let list = vec![1];
        let mut avg_col = AveragedCollection::new(list);
        avg_col.add(2);
        avg_col.add(3);
        assert_eq!(avg_col.average(), 2.0);
    }
}