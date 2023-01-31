package config

import (
	"log"
	"os"
	"strconv"
)

type Config struct {
	Port     int
	DataPath string
}

func Load() Config {
	port, err := strconv.Atoi(os.Getenv("PORT"))
	if err != nil {
		log.Println("WARNING: ENV PORT is not set. Use default \"8080\".")
		port = 8080
	}

	dataPath := os.Getenv("DATA_PATH")
	if len(dataPath) == 0 {
		log.Println("WARNING: ENV DATA_PATH is not set. Use default \"../data/\".")
		dataPath = "../data/"
	}

	return Config{port, dataPath}
}
