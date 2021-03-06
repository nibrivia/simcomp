use radix_heap::RadixHeapMap;
use std::cmp::Ordering;
use crate::nic;

pub enum EventType {
    NICRx {nic: usize, packet: nic::Packet},
    NICEnable { nic: usize },
}

pub struct Event {
    pub time: u64,
    pub event_type: EventType,
    //function: Box<dyn FnOnce() -> ()>,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
impl Eq for Event {} // don't use function

pub struct Scheduler {
    time: u64,
    limit: u64,
    //queue: BinaryHeap<Event>,
    queue: RadixHeapMap<i64, Event>,

    // network elements
    NICs: Vec<nic::NIC>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let mut nics = Vec::new();
        nics.push(nic::NIC::new());

        Scheduler {
            time : 0,
            limit: 10_000_000_000,
            //queue : BinaryHeap::new(),
            queue : RadixHeapMap::new(),

            NICs: nics,
        }
    }

    pub fn call_in(&mut self, delay: u64, event_type: EventType) {
        self.call_at(self.time+delay, event_type)
    }

    pub fn call_at(&mut self, time: u64, event_type : EventType) {
        let event = Event { time: time, event_type: event_type};
        self.queue.push(-(time as i64), event);
        //println!("will do thing at {}", time)
    }

    pub fn run(&mut self) {
        while self.queue.len() > 0 && self.time < self.limit {
            let tuple = self.queue.pop().unwrap();
            let event = tuple.1;
            self.time = event.time;

            let events = match event.event_type {
                EventType::NICRx {nic, packet} => self.NICs[nic].enq(self.time, &mut self.queue, packet),
                EventType::NICEnable {nic} => self.NICs[nic].send(self.time, &mut self.queue, true),
            };


        }
        println!("{}", self.NICs[0].count);
    }
}
