struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
       Counter { count: 0 } 
    }
}

// 自定义迭代器，主要实现 Iterator trait
impl Iterator for Counter {
    // 定义迭代器的返回类型(关联类型)
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}       


pub fn using_other_iterator_trait_mothods() {
    let sum : u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum = {}", sum);
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn iterator_sum() {
        let mut Counter = Counter::new();
        assert!(Counter.next() == Some(1));
        assert!(Counter.next() == Some(2));
        assert!(Counter.next() == Some(3));
        assert!(Counter.next() == Some(4));
        assert!(Counter.next() == Some(5));
        assert!(Counter.next() == None);
    }
}
