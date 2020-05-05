use std::collections::BinaryHeap;
use std::cmp::Ordering;
// use std::cmp::Reverse;

struct Event {
    time: u64,
    function: Box<dyn FnOnce() -> ()>,
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
    queue: BinaryHeap<Event>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            time : 0,
            limit: 1000,
            queue : BinaryHeap::new()
        }
    }

    pub fn call_in(&mut self, delay: u64, call: Box<dyn FnOnce() -> ()>) {
        self.call_at(self.time+delay, call)
    }

    pub fn call_at(&mut self, time: u64, call: Box<dyn FnOnce() -> ()>) {
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
    queue: Vec<Packet>,

    scheduler: Box<Scheduler>
}

impl NIC {
    pub fn new(s: Box<Scheduler>) -> NIC {
        NIC {
            latency_ns: 10,
            ns_per_byte: 1,
            enabled: false,
            count : 0,
            queue : Vec::new(),
            scheduler: s,
        }
    }

    pub fn enq(&mut self, p: Packet) {
        println!("Received packet!");

        self.queue.push(p);
        self.count += 1;

        // attempt send
        self.send(false);
    }

    pub fn send(&mut self, enable: bool) {
        self.enabled = self.enabled | enable;
        if !self.enabled || self.queue.len() == 0 {
            return
        }

        let p = self.queue.remove(0);
        self.enabled = false;

        let reenable = || {self.send(true)};
        self.scheduler.call_in(1500, Box::new(reenable));
    }
}


fn main() {
    let mut s = Scheduler::new();

    let f = Flow::new();
    for packet in f {
        //println!("Packet#{} {}->{}", packet.seq_num, packet.src, packet.dst);
        let t = packet.seq_num;
        let print_packet = || { let p = packet; println!("Packet#{} {}->{}", p.seq_num, p.src, p.dst); };
        s.call_in(t, Box::new(print_packet));
    }


    /*
    let test_ref = &mut || {hello("10".to_string())};
    s.call_in(10, test_ref);

    //let test_ref = &mut test;
    let test_ref = &mut || {hello("1".to_string())};
    s.call_in(1, test_ref);
    */

    s.run();

    println!("done");
}

