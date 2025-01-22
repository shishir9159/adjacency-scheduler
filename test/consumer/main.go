package main

import (
	"log"
	"sync"
	"time"

	"github.com/gofiber/fiber/v3"
)

type requestCount struct {
	sync.Mutex
	count int
}

func main() {
	app := fiber.New()
	requestCounter := &requestCount{}

	go func() {
		for {
			time.Sleep(1 * time.Second)
			requestCounter.Lock()
			log.Printf("Requests per second: %d", requestCounter.count)
			requestCounter.count = 0
			requestCounter.Unlock()
		}
	}()

	app.Post("/receive", func(c *fiber.Ctx) error {
		requestCounter.Lock()
		requestCounter.count++
		requestCounter.Unlock()

		var request struct {
			Message string `json:"message"`
		}

		if err := c.BodyParser(&request); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
				"error": "Invalid JSON",
			})
		}

		return c.JSON(fiber.Map{
			"status":  "success",
			"message": request.Message,
		})
	})

	app.Listen(":8080")
}
