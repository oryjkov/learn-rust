const MOD: usize = 17;
struct HMap {
    data: Vec<Vec<(usize, String,)>>,
}

impl HMap {
    fn insert(&mut self, key: usize, v: String)-> bool {
        let hash = key % MOD;
        if self.data.len() <= hash {
            return false;
        }
        let r = &mut self.data[hash];
        for e in r.iter_mut() {
            if e.0 == key {
                e.1 = v;
                return true;
            }
        }
        r.push((key, v));

        true
    }

    fn get(&self, key: usize) -> Option<String> {
        let hash = key % MOD;
        if self.data.len() <= hash {
            return None;
        }
        for (k, v) in &self.data[hash] {
            if *k == key {
                return Some(v.clone());
            }
        }

        None
    }

    fn print(&self) {
        for r in &self.data {
            for (k, v) in r {
                println!("{}: {}", k, v);
            }
        }
    }
}

fn main() {
    let mut hm = HMap{
        data: vec![vec![]; MOD],
    };
    hm.insert(10, "ten".to_string());
    hm.insert(11, "eleven".to_string());
    hm.insert(10, "ten_".to_string());
    hm.print();
    println!("get 10: {:?}", hm.get(10));
    println!("get 120: {:?}", hm.get(120));
    println!("get 27: {:?}", hm.get(27));
}
