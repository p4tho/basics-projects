package database

type Agent struct {
    Id             		int    	`json:"id"`
    Name           		string 	`json:"name"`
    System_name       	string 	`json:"system_name"`
    Hostname       		string 	`json:"hostname"`
    Os             		string 	`json:"os"`
    Os_version      	string 	`json:"os_version"`
    Kernel_version  	string 	`json:"kernel_version"`
    Cpu            		string 	`json:"cpu"`
}

type Beacon struct {
	Id					int
	Agent_id			int
	Timestamp			string
}

type Command struct {
	Id					int		`json:"id"`
	Agent_id			int		`json:"agent_id"`
	Command_type		int		`json:"command_type"`
	Cmd					string	`json:"cmd"`
	Status				string	`json:"status"`
	Result				string	`json:"result"`
}

type CommandResult struct {
	Command_id			int		`json:"command_id"`
	Result				string	`json:"result"`
}