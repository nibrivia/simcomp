package main

import "fmt"

func main() {
	InitQueue()

	fmt.Println("setup...")
	nic := MakeNIC()
    flow := MakeFlow()

    for {
        pkt := flow.NextPacket()
        if pkt == nil {
            break
        }
        loop.CallIn(0, func() { nic.Enque(pkt) } )
    }

    // start
	fmt.Println("start...")
	loop.Run()

    fmt.Println(nic.count, " packets processed")
	fmt.Println("done")
}
