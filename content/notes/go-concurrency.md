---
title: Golang Concurrency 
date: 4 July 2026
---

![img](https://safebooru.org//samples/836/sample_be2a388408d3904afc47e4b17b757582b5b400ad.jpg?6919917)

### goroutines

the most simple go routine can be done as 

```go
func a(string s) {
	fmt.println(a)
}

func main() {
	go a("a")
	go a("b")
	go a("c")

	do_something()
}
```

as soon as `go a()` is executed, a new `goroutine` is created and it just runs async from the `main()` function. the `main()` goes back to executing the next statement without caring about the results of the `goroutine`.

<br>

since these processes are just generated off from main, if main completes executing before the `go a()`, we would not see the result of that goroutine.


```go
func a(string s) {
	fmt.println(a)
}

func main() {
	go a("a")
}
```

this code will often print nothing since main terminates immediately.

### channels

used to communicate data between goroutines. since the `main` function is a goroutine, channels can be used to get informtion from any `goroutine inside it`.

```go
func main() {
	channel := make(chan string)
	go func() {
		channel <- data
	}()

	msg := <- channel
	fmt.Println(msg)
}
```

`select` is going to block until one of the channels inside it return something, and then executes the corresponsing block. if both return at the same time, it is selected at random

```go
select {
	case msgfromchannel1 := <- channel1:
		fmt.Println("channel 1")
	case msgfromchannel2 := <- channel2:
			fmt.Println("channel 2")
}
```
