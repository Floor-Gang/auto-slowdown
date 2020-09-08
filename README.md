# Auto Slow down

## Setup
Download [Rust](https://www.rust-lang.org/)
```shell script
$ git clone <repository url>
$ cd ./auto-slow-down
$ cargo build --release
$ cd ./target/release/auto-slow-down
$ ./auto-slow-down
# ... edit config.yml ...
$ ./auto-slow-down
```
 
## Bot Usage
### `;exclude <channel id>`
This command will exclude a given channel from being monitored by the slow-down bot.i

### `;rmexclude <channel_id`
This will remove the given channel from the excluded list and will now be monitored again.

### `;list_excluded`
This will display all of the channels that are currently excluded from being monitored.

### `;toggle`
This will toggle the state of the bot between watching channels and not watching channels.