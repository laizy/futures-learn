package main

import (
	"net"
	"time"
)

func main() {
	conn, err := net.Dial("tcp", "localhost:8080")
	if err != nil {
		panic(err)
	}

	N := 100000000
	for i := 0; i < N; i++ {
		conn.Write([]byte("aaaaaaaa"))
		time.Sleep(time.Microsecond)
	}

}
