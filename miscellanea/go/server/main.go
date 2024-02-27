package main

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Get("/*", func(c *fiber.Ctx) error {
		fmt.Println("Request URL: ", c.OriginalURL())
		fmt.Println(c)
		return c.SendString("Hello, World!")
	})

	app.Listen(":3000")
}
