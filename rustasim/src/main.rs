use std::collections::BinaryHeap;
use std::cmp::Ordering;
// use std::cmp::Reverse;

struct Event<'a> {
    time: u64,
    function: &'a mut dyn FnMut() -> (),
}

impl Ord for Event<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Event<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
impl Eq for Event<'_> {} // don't use function

struct Scheduler<'a> {
    time: u64,
    limit: u64,
    queue: BinaryHeap<Event<'a>>,
}

impl<'a> Scheduler<'a> {
    pub fn new() -> Scheduler<'a> {
        Scheduler {
            time : 0,
            limit: 1000,
            queue : BinaryHeap::new()
        }
    }

    pub fn call_in(&mut self, delay: u64, call: &'a mut dyn FnMut() -> ()) {
        self.call_at(self.time+delay, call)
    }

    pub fn call_at(&mut self, time: u64, call: &'a mut dyn FnMut() -> ()) {
        let event = Event { time: time, function: call};
        self.queue.push(event);
        println!("will do thing at {}", time)
    }

    pub fn run(&mut self) {
        println!("Start!");
        while self.queue.len() > 0 && self.time < self.limit {
            let event = self.queue.pop().unwrap();
            self.time = event.time;

            (event.function)();
        }
    }
}


fn hello(s: String) {
    println!("hi {}!", s);
}

fn main() {
    let mut s = Scheduler::new();

    //s.call_in(10, "test 10".to_string());
    //s.call_in(1, "test  1".to_string());
    //println("{}", test);
    let test_ref = &mut || {hello("10".to_string())};
    s.call_in(10, test_ref);

    //let test_ref = &mut test;
    let test_ref = &mut || {hello("1".to_string())};
    s.call_in(1, test_ref);

    s.run();

    println!("done");
}

