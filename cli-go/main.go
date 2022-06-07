package main

import "time"

type User struct {
	FirstName string   `json:"first_name"`
	LastName  string   `json:"last_name"`
	Text      string   `json:"text"`
	Year      int64    `json:"year"`
	Happy     bool     `json:"happy"`
	Skills    []string `json:"skills"`
}

func run() {
	mem := make([]User, 0)
	for i := 0; i <= 20_000_000; i++ {
		mem = append(mem, User{
			FirstName: "sergii",
			LastName:  "onufriienko",
			Year:      int64(i),
			Skills:    []string{"node.js", "go", "rust", "aws", "k8s"},
			Happy:     true,
			Text:      "heppy text",
		})

	}
}

func main() {
	run()

	println("Done")
	time.Sleep(60 * time.Second)
	println("End")
}
