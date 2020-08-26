package internal

import (
	util "github.com/Floor-Gang/utilpkg"
	dg "github.com/bwmarrin/discordgo"
)

// Bot structure
type Bot struct {
	Client *dg.Session
	Config *Config
	Database *Controller
}

// Start starts discord client, configuration and database
func Start() {
	var err error

	botConfig := getConfig()

	database := GetController("./channels.db")

	client, err := dg.New("Bot " + botConfig.Token)

	if err != nil {
		panic(err)
	}

	bot := Bot{
		Client: client,
		Config: &botConfig,
		Database: database,
	}

	client.AddHandlerOnce(bot.onReady) // This will call onReady only once
	client.AddHandler(bot.onMessage)   // This will catch all the new messages that the bot can see

	if err = client.Open(); err != nil {
		util.Report("Was an authentication token provided?", err)
	}
}
