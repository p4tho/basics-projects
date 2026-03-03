package handlers

import (
	"log"
	"encoding/json"
	"net/http"

	"server/database"
)

func AddCommand(w http.ResponseWriter, req *http.Request) {
	// Only accept post requests
    if req.Method != http.MethodPost {
		log.Println("Failed to add command - invalid method")
        http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
        return
    }

    var command database.Command

    // Parse JSON from request body
    err := json.NewDecoder(req.Body).Decode(&command)
    if err != nil {
		log.Println("Failed to add command - invalid JSON request")
        http.Error(w, "Invalid command JSON request", http.StatusBadRequest)
        return
    }

	// Add command to queue
	err = database.InsertCommand(command)
	if err != nil {
		log.Printf("Failed to add command - %s", err)
        http.Error(w, "Failed to add command to task queue", http.StatusBadRequest)
		return
	}

	// Log command event
	log.Printf("Added command {type: %d, cmd: %s} to beacon %d", command.Command_type, command.Cmd, command.Agent_id)

	w.WriteHeader(http.StatusCreated)
}

func GetAllCommands(w http.ResponseWriter, req *http.Request) {
	rows, err := database.DB.QueryContext(req.Context(), `
		SELECT id, agent_id, command_type, cmd, status, result
		FROM commands
		ORDER BY id ASC
	`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	// Initialize as empty slice so JSON = [] not null
	commands := make([]database.Command, 0)

	for rows.Next() {
		var c database.Command
		if err := rows.Scan(
			&c.Id,
			&c.Agent_id,
			&c.Command_type,
			&c.Cmd,
			&c.Status,
			&c.Result,
		); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		commands = append(commands, c)
	}

	if err := rows.Err(); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(commands)
}

func GetAllAgents(w http.ResponseWriter, req *http.Request) {
	rows, err := database.DB.QueryContext(req.Context(), `
		SELECT id, name, system_name, hostname, os, os_version, kernel_version, cpu
		FROM agents
		ORDER BY id ASC
	`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	// Initialize as empty slice so JSON = [] not null
	agents := make([]database.Agent, 0)

	for rows.Next() {
		var a database.Agent
		if err := rows.Scan(
			&a.Id,
			&a.Name,
			&a.System_name,
			&a.Hostname,
			&a.Os,
			&a.Os_version,
			&a.Kernel_version,
			&a.Cpu,
		); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		agents = append(agents, a)
	}

	if err := rows.Err(); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(agents)
}