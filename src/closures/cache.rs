use std::collections::HashMap;

// 可以缓存闭包运行结果的结构体
// 
pub struct Cache<T, U, R> 
    where T : Fn(U) -> R,
        U : Eq + std::hash::Hash + Clone, 
        R : std::clone::Clone,
    {
        calculate: T,
        cache: HashMap<U, R>, 
    }

impl <T, U, R> Cache<T, U, R> 
    where T: Fn(U) -> R,
        U : Eq + std::hash::Hash + Clone, 
        R : std::clone::Clone,
    {

    fn new(calculate: T) -> Cache<T, U, R> {
        Cache {
            calculate,
            cache: HashMap::new(),
        }
    }
    pub fn value (&mut self, args: U) -> R {
        if self.cache.contains_key(&args) {
             self.cache.get(&args).unwrap().clone()
        }else {
            let key = args.clone();
            let result = (self.calculate)(args);
            self.cache.insert(key, result.clone());
            result
        } 
        
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn tset_cache_result() {
        let mut cache = super::Cache::new(|x| x);
        let result = cache.value(1);
        assert_eq!(result, 1);
        let result = cache.value(2);
        assert_eq!(result, 2);  
    }
}

