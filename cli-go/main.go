package main

import (
	"encoding/json"
)

type User struct {
	FirstName string   `json:"first_name"`
	LastName  string   `json:"last_name"`
	Text      string   `json:"text"`
	Year      int64    `json:"year"`
	Happy     bool     `json:"happy"`
	Skills    []string `json:"skills"`
}

func main() {
	for i := 0; i <= 10_000_000; i++ {
		jsonStr :=
			"{\"first_name\": \"sergii\",\"last_name\": \"onufriienko\",\"year\": 2022,\"skills\": [\"node.js\", \"go\", \"rust\", \"aws\", \"k8s\"],\"happy\": true,\"text\": \"heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text\"}"

		var u User
		json.Unmarshal([]byte(jsonStr), &u)

	}

	// println("Done")
	// time.Sleep(60 * time.Second)
	// println("End")
}
