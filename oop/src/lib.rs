pub struct AveragedCollection {
    list: Vec<f32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.,
        }
    }

    pub fn add(&mut self, item: f32) {
        self.list.push(item);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<f32> {
        match self.list.pop() {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average =
            self.list.iter().map(|item| *item as f64).sum::<f64>() / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let mut avg_lst = AveragedCollection::new();
        avg_lst.add(3.);
        avg_lst.add(2.);
        assert_eq!(avg_lst.average(), 2.5)
    }

}
