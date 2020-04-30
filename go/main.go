package main

import "fmt"

func main() {
	nic := MakeNIC()
	fmt.Printf("Hey %#v\n", nic)

	InitQueue()

	fmt.Println("call...")
	FnA()
	FnB()

	fmt.Println("run...")
	loop.Run()

	fmt.Println("done")
}
