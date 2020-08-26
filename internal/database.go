package internal

import (
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"log"
	"os"
)

func GetController(location string) *Controller {
	if _, err := os.Stat(location); err != nil {
		_, err := os.Create(location)

		if err != nil {
			panic(err)
		}
	}
	db, err := sql.Open("sqlite3", location)

	if err != nil {
		panic(err)
	} else {
		controller := Controller{db: db}
		if err = controller.init(); err != nil {
			panic(err)
		}
		return &controller
	}
}

// This initializes the database table if it doesn't exist
func (c Controller) init() error {
	Table := "CREATE TABLE IF NOT EXISTS \"Channels\" ( \"channel_id\" TEXT, \"messages_per_second\" INTEGER, PRIMARY KEY(\"channel_id\"))"

	_, err := c.db.Exec(Table)

	return err
}

func (c Controller) getChannelData(channelId string) (bool, ChannelData) {
	var channel ChannelData

	row, err := c.db.Query(fmt.Sprintf("SELECT * FROM Channels WHERE \"channel_id\" = %s", channelId))
	if err != nil {
		log.Fatal(err)
	}
	defer row.Close()

	for row.Next() {
		err = row.Scan(&channel.ChannelId, &channel.MessagesPerSecond)
		if err != nil {
			fmt.Println(err)
		}
		return true, channel // should only be one instance of this therefore, it can just return in the first iteration
	}

	return false, ChannelData{} // return default values as its obvious the current channel is not in the database
}

func (c Controller) getAllChannels() []ChannelData {
	var channels []ChannelData

	row, err := c.db.Query("SELECT * FROM Channels")
	if err != nil {
		log.Fatal(err)
	}

	defer row.Close()

	for row.Next() {
		var channel ChannelData
		err = row.Scan(&channel.ChannelId, &channel.MessagesPerSecond)
		if err != nil {
			fmt.Println(err)
		}
		channels = append(channels, channel)
	}

	return channels
}

func (c Controller) updateChannel(channelData ChannelData) {
	updateQuery := fmt.Sprintf(fmt.Sprintf("UPDATE Channels SET \"messages_per_second\" = %d WHERE \"channel_id\" = %s", channelData.MessagesPerSecond, channelData.ChannelId))

	var err error
	tx, _ := c.db.Begin()
	_, err = tx.Exec(updateQuery)
	if err != nil {
		log.Fatal(err)
	}
	err = tx.Commit()
	if err != nil {
		log.Fatal(err)
	}
}

func (c Controller) addNewChannel(channelData ChannelData) {
	insertQuery := fmt.Sprintf("INSERT INTO Channels (\"channel_id\", \"messages_per_second\") VALUES(%s, %d)", channelData.ChannelId, channelData.MessagesPerSecond)

	var err error
	tx, _ := c.db.Begin()
	_, err = tx.Exec(insertQuery)
	if err != nil {
		log.Fatal(err)
	}
	err = tx.Commit()
	if err != nil {
		log.Fatal(err)
	}
}
