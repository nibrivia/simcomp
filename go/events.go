package main

import "fmt"
import "container/heap"

type Event struct {
	time int
	//index  int
	action func()
}

type PriorityQueue []*Event

type EventLoop struct {
	time  int
	limit int

	stop bool

	queue PriorityQueue
}

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].time < pq[j].time
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*Event)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // avoid memory leak
	*pq = old[0 : n-1]
	return item
}

var loop *EventLoop

func InitQueue() {
	loop = &EventLoop{}

	loop.stop = false

	// heap
	loop.queue = make(PriorityQueue, 0)
	heap.Init(&loop.queue)

	loop.limit = 100
}

func (loop *EventLoop) Run() {
	for {

		// check that things are okay
		if loop.queue.Len() == 0 {
			fmt.Println("no more events")
			return
		}
		if loop.stop {
			fmt.Println("Stop received")
			return
		}

		// find the next event, update state
		event := heap.Pop(&loop.queue).(*Event)
		loop.time = event.time

		if loop.time > loop.limit {
			fmt.Println("past time limit")
			return
		}

		// run the event
		//fmt.Printf("%d exec %#v\n", loop.time, event)
		event.action()
	}
}

func (loop *EventLoop) CallIn(delay int, fn func()) {
	loop.ScheduleAt(loop.time+delay, fn)
}

func (loop *EventLoop) ScheduleAt(time int, fn func()) {
	e := Event{
		time:   time,
		action: fn,
	}

	heap.Push(&loop.queue, &e)
}

func fnA() {
	fmt.Println(loop.time, " AA")
	FnA()
}

func FnA() {
	wrap := func() { fnA() }
	loop.CallIn(11, wrap)
	loop.CallIn(30, wrap)
}

func fnB() {
	fmt.Println(loop.time, " B")
	FnB()
}

func FnB() {
	wrap := func() { fnB() }
	loop.CallIn(1, wrap)
}
