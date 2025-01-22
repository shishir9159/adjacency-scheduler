package main

import (
	"bytes"
	"fmt"
	"os"
	"net/http"
	"sync"
	"time"
)

func main() {

// set the pod name in the config map

	url := os.Getenv("URL")
	payload := []byte(`{
	    "message": "hello",
	}`)

	requestsPerSecond := 1000

	// Create a WaitGroup to manage goroutines
	var wg sync.WaitGroup

	ticker := time.NewTicker(time.Second / time.Duration(requestsPerSecond))
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			wg.Add(1)

			go func() {
				defer wg.Done()

				resp, err := http.Post(url, "application/json", bytes.NewBuffer(payload))
				if err != nil {
					fmt.Printf("Request failed: %v\n", err)
					return
				}
				defer resp.Body.Close()

				fmt.Printf("Response status: %s\n", resp.Status)
			}()
		}
	}
}
