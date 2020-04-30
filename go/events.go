package main

import "fmt"
//import "sync"
import "container/heap"

type Event struct {
	time int64
	//index  int
	action func()
}

type PriorityQueue []*Event

type EventLoop struct {
	time  int64
	limit int64

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
	//old[n-1] = nil // avoid memory leak
	*pq = old[0 : n-1]
	return item
}

var loop *EventLoop

func InitQueue() {
	loop = &EventLoop{}

	loop.stop = false

	// heap
	loop.queue = make(PriorityQueue, 0, 1000)
	heap.Init(&loop.queue)

	loop.limit = 10e9
}

func (loop *EventLoop) Time() string {
    time_ms := loop.time/1e6
    time_us := loop.time/1e3 % 1e3
    time_ns := loop.time % 1e3
    return fmt.Sprintf("%5d.%3d.%3d", time_ms, time_us, time_ns)
}

func (loop *EventLoop) Run() {
    var event *Event
	for {
		// check that things are okay
		if loop.queue.Len() == 0 {
			fmt.Println(loop.Time(), " no more events")
			return
		}

		// find the next event, update state
		event = heap.Pop(&loop.queue).(*Event)
		loop.time = event.time

		if loop.time > loop.limit {
			fmt.Println(loop.Time(), " past time limit")
			return
		}

		// run the event
        //fmt.Printf("%v exec %#v\n", loop.Time(), event)
        event.action()
	}
}

func (loop *EventLoop) CallIn(delay int64, fn func()) {
	e := Event{
		time:   loop.time+delay,
		action: fn,
	}

	heap.Push(&loop.queue, &e)
}

func (loop *EventLoop) ScheduleAt(time int64, fn func()) {
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
