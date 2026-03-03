package database

import (
	"os"
	"log"
	"database/sql"
	_ "github.com/mattn/go-sqlite3"
)

var (
	DB *sql.DB
)

// Initialize database
func DB_Init() {
	var err error
	var sql_statements []byte

	// Start database and DB handler
	DB, err = sql.Open("sqlite3", "./c2.db")
	if err != nil {
		log.Fatal(err)
	} else if err := DB.Ping(); err != nil {
        log.Fatal(err)
    }

	// Create tables
	sql_statements, err = os.ReadFile("database/sql/tables.sql")
	if err != nil {
		log.Fatal(err)
	} else if _, err = DB.Exec(string(sql_statements)); err != nil {
		log.Fatal(err)
	}
}