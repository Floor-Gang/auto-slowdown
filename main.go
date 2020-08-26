package main

import (
	"github.com/Floor-Gang/auto-slowdown/internal"
	util "github.com/Floor-Gang/utilpkg"
)

func main() {
	internal.Start()
	util.KeepAlive()
}
