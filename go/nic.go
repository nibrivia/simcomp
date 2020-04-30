package main // TODO figure out how this works...

//import "fmt"
import "github.com/gammazero/deque"

type Receiver interface {
    Receive(*Packet)
}

type NIC struct {
	uid      int
	delay_ns int64
    bandwidth_nspB int

    queue deque.Deque

    count int

    enabled bool
}

func (nic *NIC) Enque(pkt *Packet) {
	//fmt.Println(pkt)

    // update state
    nic.queue.PushBack(pkt)

    // attempt send
    if nic.enabled {
        nic.Start(false)
    }
}


func (nic *NIC) Start(enabled bool) {
    nic.enabled = enabled || nic.enabled
    if !nic.enabled || nic.queue.Len() == 0 {
        return
    }

    packet := nic.queue.PopFront().(*Packet)


    //fmt.Println(loop.Time(), " packet tx ", packet)

    // disable
    nic.enabled = false
    loop.CallIn(1500, func() { nic.Start(true) })

    // call the rx function
    nic.rx(packet, 1510)
}

func (nic *NIC) rx(pkt *Packet, delay int64) {
    w := func() {
        nic.Enque(pkt)
        nic.count++
    }
    loop.CallIn(delay, w)
}

func MakeNIC() *NIC {
	nic := &NIC{
        delay_ns: 10,
        bandwidth_nspB : 1,
        enabled: true,
    }
	return nic
}
