package internal

import (
	"log"
	"strings"

	dg "github.com/bwmarrin/discordgo"
)

func (bot *Bot) onMessage(_ *dg.Session, msg *dg.MessageCreate) {
	// check if they provided a prefix and they're not a bot
	if msg.Author.Bot || !strings.HasPrefix(msg.Content, bot.Config.Prefix) {
		return
	}

	// let's split their message up into arguments
	// args = [prefix, sub-command name]
	args := strings.Fields(msg.Content)

	if len(args) < 2 { // this would mean args is [prefix] which at that point ignore them
		return
	}

	// we can now find out what command they were calling
	switch args[1] {
	case "ping":
		bot.cmdPing(msg.Message)
		break
	}
}

func (bot *Bot) onReady(_ *dg.Session, ready *dg.Ready) {
	log.Printf("<Bot/Feature Name> - ready as %s#%s", ready.User.Username, ready.User.Discriminator)
}
