mod test;

pub struct Queue<T: PartialOrd> {
    items: Vec<T>,
}

impl<T: PartialOrd> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }
    pub fn push(self: &mut Queue<T>, item: T) {
        self.items.push(item);
        let mut i = self.items.len() - 1;
        while i != 0 {
            let parent = (i - 1) / 2;
            if !(self.items[parent] > self.items[i]) {
                break;
            }
            self.items.swap(parent, i);
            i = parent;

        }
    }
    pub fn len(self: &Queue<T>) -> usize {
        self.items.len()
    }
    pub fn peek(self: &Queue<T>) -> Option<&T> {
        if self.items.len() == 0 {
            None
        } else {
            Some(&self.items[0])
        }
    }
    pub fn pop(self: &mut Queue<T>) -> Option<T> {
        if self.items.len() == 0 {
            return None;
        }
        let n = self.items.swap_remove(0);
        let mut i = 0;
        loop {
            let mut smallest = i;
            let left = i * 2 + 1;
            let right = i * 2 + 2;
            if left < self.items.len() && self.items[left] <= self.items[smallest] {
                smallest = left
            }
            if right < self.items.len() && self.items[right] <= self.items[smallest] {
                smallest = right
            }
            if smallest == i {
                break;
            }
            self.items.swap(smallest, i);
            i = smallest;
        }
        Some(n)
    }
}
