# rescrobbled-maloja

A fork of [rescrobbled](https://github.com/InputUsername/rescrobbled) to scrobble to a selfhosted [Maloja](https://github.com/krateng/maloja) server in addition to Last.fm. All scrobbling features for Listenbrainz.org have been replaced with Maloja.

I used code from [listenbrainz-rust 0.2.2
](https://crates.io/crates/listenbrainz-rust) as the foundation for scrobbling to Maloja. I couldn't find any easy and universal way to scrobble to Maloja from Linux so I'm lucky to have found rescrobbled.

*I edited the original README below to fit this fork.*

---

Rescrobbled is a music scrobbler daemon written in Rust. Rescrobbled detects active media players running on D-Bus using `MPRIS`, automatically updates "now playing" status, and scrobbles songs as they play.

# How to install and use

You can download one of the prebuilt binaries [here](https://github.com/code-ink-space/rescrobbled-maloja/releases). The binary can be placed anywhere you like.

Alternatively you can install from source using `cargo install --path .` from the crate root.

To use rescrobbled-maloja, you'll need a Last.fm API key and secret. These can be obtained [here](https://www.last.fm/api/account/create) and an API key from your Maloja server to be used as a token.

## Configuration

Rescrobbled expects a configuration file at `~/.config/rescrobbled/config.toml` with the following format:
```toml
api-key = "Last.fm API key"
api-secret = "Last.fm API secret"
lb-token = "Maloja API key"
enable-notifications = false # optional
#min-play-time = { secs: 0, nanos: 0 } # optional
malojaserver = "https://<YOUR MALOJA SERVER>/api/newscrobble"
```
By default, track submission respects Last.fm's recommended behavior; songs should only be scrobbled if they have been playing for at least half their duration, or for 4 minutes, whichever comes first. Using `min-play-time` you can override this.

## Running rescrobbled

To make sure that rescrobbled can scrobble to Last.fm, you need to run the program in a terminal. This will prompt you for your Last.fm username and password, and authenticate with Last.fm. A long-lasting session token is then obtained, which will be used on subsequent runs instead of your username/password.

If you want to run rescrobbled as a daemon, you can put the provided [systemd unit file](https://github.com/code-ink-space/rescrobbled-maloja/blob/master/rescrobbled.service) in the `~/.config/systemd/user/` directory.
Change `ExecStart` to point to the location of the binary, as necessary. Then, to enable the program to run at startup, use:
```
systemctl --user enable rescrobbled.service
```
You can run it in the current session using:
```
systemctl --user start rescrobbled.service
```
