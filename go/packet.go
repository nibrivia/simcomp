package main

type Packet struct {
	src     int
	dst     int
	seq_num int
    size_byte int

	ttl     int
	sent_ts int64
}

type Flow struct {
	src        int
	dst        int
	size_bytes int

	cwnd int

	next_seq int
}

const BYTES_PER_PACKET = 1500

func (flow *Flow) NextPacket() *Packet {
	// stop if we're past done
	if flow.next_seq*BYTES_PER_PACKET > flow.size_bytes {
		return nil
	}

	// create packet and update state
	p := &Packet{
		seq_num: flow.next_seq,
		src:     flow.src,
		dst:     flow.dst,
		ttl:     10,
		sent_ts: loop.time,
        size_byte : BYTES_PER_PACKET,
	}
	flow.next_seq++

	return p
}

func MakeFlow() *Flow {
	flow := &Flow{}
	flow.size_bytes = BYTES_PER_PACKET * 200

	return flow
}
