# cunny-rs

cunny (short for crunchy and funny), is a tool to download video file from crunchyroll.

## Dependencies

- ffmpeg

This project was only tested on linux, but in theory everything should work the same on windows, just make sure that ffmpeg is installed correctly on the system path.

## Downloading

At this time there is no pre-compiled binaries for cunny, so you must clone the repo and compile it yourself.

```bash
git clone https://github.com/Gonzera/cunny-rs.git
cd cunny-rs
cargo build --release
```

Keep in mind that you need to have rust toolchain installed in your system.

## Usage

Cunny is a a very simple cli tool, so there's no GUI, but the list of options is really small.

**Options**

```bash
  -u, --user <USER>                  Crunchyroll username or email
  -p, --password <PASSWORD>          Crunchyroll password
  -d, --directory <DIRECTORY>        Save directory, by the default it will create a new directory with the show's title Use this option to overwrite this behavior
  -s, --show-id <SHOW_ID>            ID of the show This option will download all seasons and episodes
  -e, --episode-id <EPISODE_ID>      ID of the episode This option will download only a single episode
      --locale <LOCALE>              [default: en-US]
      --audio-locale <AUDIO_LOCALE>  [default: ja-JP]
      --substitles <SUBSTITLES>      [default: en-US]
  -h, --help                         Print help
  -V, --version                      Print version
```

**Example usage**

Downloading an entire show: `cunny-rs -u youremail@email.com -p yourpassword -s GNVHKNPQ7`

Downloading a single episode: `cunny-rs -u youremail@email.com -p yourpassword -e GEVUZM77J`

### Where can i find the show or episode id?

Go to the page of the show you want to download, the url should be as following: `series/XXXXXXX/show-name`. the id will be the XXXXXXX part. For single episodes, just go to the watch page and use the same logic.

## Disclaimer

The code provided in this repository is intended for research and educational purposes only. It is designed to demonstrate the technical feasibility of downloading videos from a streaming platform. The code should not be used to infringe upon the copyright or terms of service of any streaming platform or to engage in any illegal activities.

Downloading videos from streaming platforms without proper authorization may violate the terms of service of those platforms and may also infringe upon the intellectual property rights of the content creators. The creator of this project does not endorse, support, or encourage any form of copyright infringement or illegal activities.

Users of this code are solely responsible for complying with all applicable laws, regulations, and terms of service when using this code. The creator of this project shall not be held liable for any misuse, damages, or legal consequences resulting from the use or misuse of this code.

Please respect the rights of content creators and only use this code for lawful purposes in accordance with the applicable laws and terms of service.
