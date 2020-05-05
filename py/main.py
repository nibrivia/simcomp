from heapq import heappush, heappop
from collections import deque

BYTES_PER_PACKET = 1500


class Loop:
    def __init__(self):
        self.time = 0
        self.queue = []
        self.limit = 10e9
        self.count = 0

    def run(self):
        while len(self.queue) > 0 and self.time < self.limit:
            self.time, _, fn, args, kwargs = heappop(self.queue)
            fn(*args, **kwargs)

    def call_in(self, delay, fn, *args, **kwargs):
        self.call_at(self.time+delay, fn, *args, **kwargs)

    def call_at(self, time, fn, *args, **kwargs):
        self.count += 1
        heappush(self.queue, (time, self.count, fn, args, kwargs))


class NIC:
    def __init__(self):
        self.enable = False
        self.ns_per_bytes = 1
        self.latency_ns = 10

        self.queue = deque()
        self.count = 0

    def enq(self, packet):
        self.queue.append(packet)
        # if self.enable:
        self.send()
        self.count += 1

    def send(self, enable=False):
        self.enable = self.enable or enable
        if not self.enable or len(self.queue) == 0:
            return

        packet = self.queue.popleft()

        # tx_delay = packet.size_byte * self.ns_per_bytes
        # rx_delay = self.latency_ns + tx_delay
        self.enable = False

        # Do the calls
        R.call_in(1500, self.send, True)
        R.call_in(1510, self.rx_fn, packet)


class Packet:
    def __init__(self, src, dst, seq_num, size_byte, ttl, sent_ts):
        self.src = src
        self.dst = dst
        self.seq_num = seq_num
        self.size_byte = size_byte

        self.ttl = ttl
        self.sent_ts = sent_ts


class Flow:
    def __init__(self, src, dst, size_byte):
        self.src = src
        self.dst = dst
        self.size_byte = size_byte

        self.cwnd = 1
        self.next_seq = 0

    def packets(self):
        bytes_sent = 0
        seq_num = 0
        while bytes_sent < self.size_byte:
            p = Packet(self.src, self.dst, seq_num, BYTES_PER_PACKET, 10, 0)
            yield p
            seq_num += 1
            bytes_sent += BYTES_PER_PACKET


def main():
    # Go
    queue = NIC()
    queue.rx_fn = queue.enq
    flow = Flow(1, 2, 200*BYTES_PER_PACKET)
    for p in flow.packets():
        queue.enq(p)

    R.call_at(0, queue.send, enable=True)
    R.run()

    print(queue.count)


R = Loop()
if __name__ == "__main__":
    main()
    print("done")
