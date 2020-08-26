package internal

import (
	dg "github.com/bwmarrin/discordgo"
	"log"
	"time"
)

func (bot *Bot) onMessage(_ *dg.Session, msg *dg.MessageCreate) {
	go bot.incrementMessageCount(msg)

	////check if they provided a prefix and they're not a bot
	//if msg.Author.Bot || !strings.HasPrefix(msg.Content, bot.Config.Prefix) {
	//	return
	//}
	//
	//// let's split their message up into arguments
	//// args = [prefix, sub-command name]
	//args := strings.Fields(msg.Content)
	//
	//if len(args) < 2 { // this would mean args is [prefix] which at that point ignore them
	//	return
	//}
	//
	//// we can now find out what command they were calling
	//switch args[1] {
	//case "ping":
	//	bot.cmdPing(msg.Message)
	//	break
	//}
}

func (bot *Bot) onReady(_ *dg.Session, ready *dg.Ready) {
	log.Printf("<Bot/Feature Name> - ready as %s#%s", ready.User.Username, ready.User.Discriminator)

	go bot.checker() // run the checker on another thread
}

func (bot *Bot) checker() {
	timer := time.Tick(time.Duration(2500) * time.Millisecond) // 1 second intervals

	for range timer {
		bot.checkChannels()
	}
}

func (bot *Bot) checkChannels() {
	for _, channel := range bot.Database.getAllChannels() {
		// Apply appropriate slow mode settings
		if between(channel.MessagesPerSecond, 51,100) {
			bot.updateSlowMode(channel, 60)
		} else if between(channel.MessagesPerSecond, 2,50) {
			bot.updateSlowMode(channel, 30)
		} else if between(channel.MessagesPerSecond, 0,1) {
			bot.updateSlowMode(channel, 0)
		}

		// Reset channel data
		channel.MessagesPerSecond = 0
		bot.Database.updateChannel(channel)
	}
}

func between(number int, min int, max int) bool {
	if min <= number && number <= max {
		return true
	}

	return false
}

func (bot *Bot) updateSlowMode(channel ChannelData, amount int) {
	var x, _ = bot.Client.Channel(channel.ChannelId)
	_, err := bot.Client.ChannelEditComplex(channel.ChannelId, &dg.ChannelEdit{
		Name:                 x.Name,
		Topic:                x.Topic,
		NSFW:                 x.NSFW,
		Position:             x.Position,
		Bitrate:              x.Bitrate,
		UserLimit:            x.UserLimit,
		PermissionOverwrites: x.PermissionOverwrites,
		ParentID:             x.ParentID,
		RateLimitPerUser:     amount,
	})

	if err != nil {
		log.Fatal(err)
	}
}

func (bot *Bot) incrementMessageCount(msg *dg.MessageCreate) {
	valid, data := bot.Database.getChannelData(msg.ChannelID)

	if valid {
		data.MessagesPerSecond += 1
		bot.Database.updateChannel(data)
	} else {
		var data = ChannelData{
			ChannelId:         msg.ChannelID,
			MessagesPerSecond: 1,
		}

		bot.Database.addNewChannel(data)
	}
}
