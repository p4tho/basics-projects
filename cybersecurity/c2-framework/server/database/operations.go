package database

// Add new agent and return its unique id
func InsertAgent(agent Agent) (int64, error) {
	stmt := "INSERT INTO agents(name, system_name, hostname, os, os_version, kernel_version, cpu) VALUES(?, ?, ?, ?, ?, ?, ?)"

	// Execute insertion
	result, err := DB.Exec(
		stmt,
		agent.Name,
		agent.System_name,
		agent.Hostname,
		agent.Os,
		agent.Os_version,
		agent.Kernel_version,
		agent.Cpu,
	)
	if err != nil {
		return -1, err
	}

	// Get id for new agent
	id, err := result.LastInsertId()
	if err != nil {
		return  -1, err
	}

	return id, nil
}

// Add commands to task queues specific to each agent
func InsertCommand(command Command) error {
	stmt := "INSERT INTO commands(agent_id, command_type, cmd) VALUES(?, ?, ?)"

	_, err := DB.Exec(
		stmt,
		command.Agent_id,
		command.Command_type,
		command.Cmd,
	)
	if err != nil {
		return err
	}

	return nil
}

func InsertBeacon(beacon Beacon) {}
