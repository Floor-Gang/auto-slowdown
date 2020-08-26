package internal

import "database/sql"

type ChannelData struct {
	ChannelId string `json:"channel_id"`
	MessagesPerSecond int `json:"messages_per_second"`
}

type Controller struct {
	db *sql.DB
}
