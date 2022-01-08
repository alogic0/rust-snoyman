// p. 418, Itrator on parametrized type

struct VecIter<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T: Copy> Iterator for VecIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            None
        } else {
            let res = Some(self.vec[self.index]);
            self.index += 1;
            res
        }
    }
}

fn main() {
    let fibs: Vec<u32> = vec![1, 1, 2, 3, 5, 8, 13];
    let iter = VecIter {
        vec: fibs,
        index: 0,
    };
    for x in iter {
        println!("{}", x);
    }
}

