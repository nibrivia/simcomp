package main // TODO figure out how this works...

import "fmt"

type NIC struct {
	uid      int
	delay_ns int
}

func (nic *NIC) Enque(pkt *Packet) {
	fmt.Println(pkt)
}

func MakeNIC() *NIC {
	nic := &NIC{delay_ns: 1000}
	fmt.Println("Hi from MakeNIC")
	return nic
}
