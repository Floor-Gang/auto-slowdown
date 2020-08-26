package main

import (
	"github.com/Floor-Gang/init-discord-bot/internal"
	util "github.com/Floor-Gang/utilpkg"
)

func main() {
	internal.Start()
	util.KeepAlive()
}
