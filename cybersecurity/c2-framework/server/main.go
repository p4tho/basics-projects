package main

import (
	"server/c2"
	"server/database"
)

func main() {
	database.DB_Init()
	defer database.DB.Close()

	c2.StartHTTP(8080)
}