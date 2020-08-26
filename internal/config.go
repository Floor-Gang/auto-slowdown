package internal

import (
	"log"

	utilConfig "github.com/Floor-Gang/utilpkg/config"
)

// Config structure.
type Config struct {
	Token  string `yaml:"token"`
	Prefix string `yaml:"prefix"`
}

const configPath = "./config.yml"

// GetConfig retrieves a configuration.
func getConfig() Config {
	config := Config{
		Token:  "",
		Prefix: ".<command name>",
	}
	err := utilConfig.GetConfig(configPath, &config)

	if err != nil {
		log.Fatalln(err)
	}

	return config
}

// Save saves configuration
func (config *Config) Save() {
	if err := utilConfig.Save(configPath, config); err != nil {
		log.Println("Failed to save config", err)
	}
}
