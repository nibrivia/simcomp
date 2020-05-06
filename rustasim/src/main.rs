use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use radix_heap::RadixHeapMap;

// use std::cmp::Reverse;

enum EventType {
    NICRx {nic: usize, packet: Packet},
    NICEnable { nic: usize },
}

struct Event {
    time: u64,
    event_type: EventType,
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

struct Scheduler {
    time: u64,
    limit: u64,
    //queue: BinaryHeap<Event>,
    queue: RadixHeapMap<i64, Event>,

    // network elements
    NICs: Vec<NIC>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let mut nics = Vec::new();
        nics.push(NIC::new());

        Scheduler {
            time : 0,
            limit: 100_000_000_000,
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



struct Packet {
    src: u32,
    dst: u32,
    seq_num: u64,
    size_byte: u64,

    ttl: u32,
    sent_ns: u64,
}

struct Flow {
    src: u32,
    dst: u32,

    size_byte: u64,
    cwnd: u32,
    next_seq: u64,
}

const BYTES_PER_PACKET: u64 = 1500;

impl Flow {
    pub fn new() -> Flow {
        Flow {
            src: 0,
            dst: 0,

            size_byte: 200*BYTES_PER_PACKET,
            cwnd: 1,
            next_seq: 0
        }
    }
}

impl Iterator for Flow {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_seq*BYTES_PER_PACKET < self.size_byte {
            let p = Packet {
                src: self.src,
                dst: self.dst,
                seq_num: self.next_seq,
                size_byte: BYTES_PER_PACKET,
                ttl: 10,
                sent_ns: 0,
            };
            self.next_seq += 1;
            Some(p)
        } else {
            None
        }
    }
}


struct NIC {
    latency_ns: u64,
    ns_per_byte: u64,
    enabled: bool,
    count: u64,
    queue: VecDeque<Packet>,
}

impl NIC {
    pub fn new() -> NIC {
        NIC {
            latency_ns: 10,
            ns_per_byte: 1,
            enabled: false,
            count : 0,
            queue : VecDeque::new(),
        }
    }

    pub fn enq(&mut self, time: u64, event_queue: &mut RadixHeapMap<i64, Event>, p: Packet) {
        //println!("Received packet #{}!", p.seq_num);

        self.queue.push_back(p);
        self.count += 1;

        // attempt send
        self.send(time, event_queue, false);
    }

    pub fn send(&mut self, time: u64, event_queue: &mut RadixHeapMap<i64, Event>, enable: bool) {
        self.enabled = self.enabled | enable;

        if !self.enabled || self.queue.len() == 0 {
            return
        }

        let packet = self.queue.pop_front().unwrap();
        //println!("Sending packet #{}", packet.seq_num);
        self.enabled = false;


        //scheduler.call_in(1500, EventType::NICEnable{nic: 0});
        //scheduler.call_in(1510, EventType::NICRx{nic: 0, packet});
        event_queue.push(-(time as i64+1500), Event{time: time+1500, event_type: EventType::NICEnable{nic: 0}});
        event_queue.push(-(time as i64+1510), Event{time: time+1510, event_type: EventType::NICRx{nic: 0, packet}});
        //let reenable = || {self.send(true)};
        //self.scheduler.call_in(1500, Box::new(reenable));
    }
}


fn main() {
    let mut s = Scheduler::new();

    let f = Flow::new();
    for packet in f {
        s.call_in(0, EventType::NICRx{nic: 0, packet});
    }
    s.call_in(0, EventType::NICEnable{nic: 0});


    s.run();

    println!("done");
}

