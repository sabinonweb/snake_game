use crate::snake::Segment;
use ggez::GameResult;

#[derive(Clone, Debug, PartialEq)]
pub struct Queue {
    pub queue: Vec<Segment>,
    pub front: isize,
    pub rear: isize,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue: Vec::new(),
            front: -1,
            rear: -1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    pub fn is_full(&self) -> bool {
        self.front == (self.rear + 1) % (self.queue.len() as isize)
    }
    
    pub fn enqueue(&self, value: Segment) -> GameResult {
        let rear = (self.rear + 1) % (self.queue.len() as isize);

        if !self.is_full() {
            self.queue.insert(rear.try_into().unwrap(), value);
        } else {
            println!("The queue is full");
        }

        Ok(())
    }

    pub fn dequeue(&self) -> GameResult {
        self.front += 1;

        if !self.is_empty() {
            self.queue.remove(self.front.try_into().unwrap());
        } else {
            println!("The queue is empty");
        }

        Ok(())
    }
}

impl Iterator for Queue {
    type Item = Segment;

    fn next(&mut self) -> Option<Segment> {
        for segment in self.queue {
            return Some(segment);
        }

        None
    }
}

