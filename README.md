# labadoor

This is the project created to use with our electronic door at τοLabάκι hackerspace, but designed to be modular and configurable. It is designed to run on an SBC, such as a RaspberryPi, and tries to not be designed around a specific setup when possible, so it can be used by others.

## Modules

Functionality is separated into modules:

* `labadoor-gpio`: Sets high/low to a GPIO pin -- meant to be connected to a relay or other external device that unlocks/opens the door
* `labadoor-csv`: Handles storage of the users' identifiers and authenticates them. (TODO: this would make sense to be renamed as `labadoor-auth`, and provide multiple backends as `--features` )
* `labadoor-matrix`,`labadoor-telegram`: Daemons for messaging bots, that allow the user to open the door by sending a message
* `labadoor-rest`: WIP, provides a REST API for managing users and opening the door

Each of them can be configured with:

* Environment variables
* A common configuration file, with sections for each module
* Command-line arguments

They are libraries, with a "main" function that can be called from frontends. There is the `labadoor` frontend, which provides a subcommand for each module and handles reading each module's configuration before called.

This way you can choose which modules are included in your setup just by setting the desired `--features` when building. Also, each of them can be used by other utilities and frontends (e.g. `labadoor-rest` will still use `labadoor-csv` to authenticate users, and simply provide an interface for relevant operations)

## Parameters

### `labadoor-gpio`

`pub fn gpio(device: String, pin: u8, active_low: bool, active_time: u32);`

* `device`: Path to the linux GPIO device to use
* `pin`: The pin to write the "open" signal to
* `active_low`: If the external device needs active low input
* `active_time`: The amount of time to keep the "open" signal active, before writting "close"

### `labadoor-csv`

`pub fn csv(path: String, method: String, identifier: String, resource_shortcut: i8);`

* `path`: The directory containing the csv files
* `method`: The authentication method (i.e. "matrix", "telegram")
* `identifier`: The identifier of the user currently requesting access (more details below)
* `resource_shortcut`: The shortcut for the requested to give access (more details below)

`identifier` is a user identifier for each authentication method. This might be a username, phone number, UUID of an access card, etc

`resource_shortcut`: Since users might have access to many resources (i.e. doors), the user needs to specify which resource they would like to unlock. The best way I found to handle this was by assigning them arbitrary numbers, which can be easily selected from a range of authentication methods (and their input methods): online messages, phone calls, on-site keypads etc.

### `labadoor-matrix`

`pub async fn matrix(username: String, password: String);`: username and password for the bot account.

### `labadoor-telegram`

`pub async fn telegram(token: String);`: Telegram API token.

## Design goals

While there are many projects written for such a use case, many of them seemed to be custom-written for a specific setup and design decision. As such, however well implemented they might be, they seemed really hard to customize for our use-case. So... we [wrote our own](https://xkcd.com/927/)!

The modular design allows the user to use the stack while customizing the features for their setup:

* `labadoor-rest` is WIP, which could allow us to create a web interface for user management
* Extra authentication methods (e.g. VoIP, messaging services, RFID) can be easily integrated, just by writting the appropriate module, and call its "main" function from `labadoor`
* Authentication methods can even be easily written in a different language! They can still integrate with every other labadoor module just by calling the `labadoor` binary.
