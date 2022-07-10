//! # Playerctl
//!
//! A simple Rust wrapper for the playerctl.
//!
//! This project is not associated with the playerctl project.
//!
//! This does not include the playerctl library, so you will need to install it yourself.
//!
//! See the [playerctl](https://github.com/altdesktop/playerctl) project for more information.

#![warn(missing_docs)]

use std::process::Command;

fn command(command: &str) -> String {
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();

    let stdout = Command::new(parts.remove(0))
        .args(parts)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute command '{}'", command))
        .stdout;

    String::from_utf8(stdout).unwrap()
}

/// The current state of the player.
#[derive(Debug, Clone, Copy)]
pub enum PlayerStatus {
    /// Media is currently playing.
    Playing,
    /// Media is currently paused.
    Paused,
    /// Media is currently stopped.
    Stopped,
}

impl From<String> for PlayerStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "Playing" => PlayerStatus::Playing,
            "Paused" => PlayerStatus::Paused,
            "Stopped" => PlayerStatus::Stopped,
            _ => panic!("Unknown player status"),
        }
    }
}

/// The player's looping state.
#[derive(Debug, Clone, Copy)]
pub enum LoopStatus {
    /// Media is not looping.
    None,
    /// The current track will loop.
    Track,
    /// All tracks will loop.
    Playlist,
}

impl From<String> for LoopStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "None" => LoopStatus::None,
            "Track" => LoopStatus::Track,
            "Playlist" => LoopStatus::Playlist,
            _ => panic!("Unknown loop status"),
        }
    }
}

/// The player's shuffle state to set.
#[derive(Debug, Clone, Copy)]
pub enum ShuffleStatus {
    /// Media will be shuffled.
    On,
    /// Media will not be shuffled.
    Off,
    /// The shuffle status will be toggled.
    Toggle,
}

impl From<String> for ShuffleStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "On" => ShuffleStatus::On,
            "Off" => ShuffleStatus::Off,
            "Toggle" => ShuffleStatus::Toggle,
            _ => panic!("Unknown shuffle status"),
        }
    }
}

/// The currently playing track's metadata.
///
/// If any of the metadata is not available, that field will be an empty string.
pub struct TrackMetadata {
    /// The track's artist.
    pub artist: String,
    /// The track's title.
    pub title: String,
    /// The track's album.
    pub album: String,
}

/// The playerctl library.
///
/// Allows you to control the player.
pub struct PlayerCtl;

impl PlayerCtl {
    /// Play media.
    pub fn play() {
        command("playerctl play");
    }

    /// Pause media.
    pub fn pause() {
        command("playerctl pause");
    }

    /// Play / pause media.
    pub fn play_pause() {
        command("playerctl play-pause");
    }

    /// Stop media.
    pub fn stop() {
        command("playerctl stop");
    }

    /// Skip to the next track.
    pub fn next() {
        command("playerctl next");
    }

    /// Skip to the previous track.
    pub fn previous() {
        command("playerctl previous");
    }

    /// Seek forwards / backwards in seconds.
    pub fn position(secs: f32) {
        if secs < 0. {
            command(&format!("playerctl position {}-", -secs));
        } else {
            command(&format!("playerctl position {}+", secs));
        }
    }

    /// Set the volume to the given percentage from 0 to 1.
    pub fn volume(percent: f32) {
        if percent < 0. {
            command(&format!("playerctl volume {}-", -percent));
        } else {
            command(&format!("playerctl volume {}+", percent));
        }
    }

    /// Gets the current player status.
    pub fn status() -> PlayerStatus {
        command("playerctl status").into()
    }

    /// Get the metadata of the currently playing track.
    pub fn metadata() -> TrackMetadata {
        let title = command("playerctl metadata title").trim().to_string();
        let artist = command("playerctl metadata artist").trim().to_string();
        let album = command("playerctl metadata album").trim().to_string();

        TrackMetadata {
            artist,
            title,
            album,
        }
    }

    /// Open the given uri in the player.
    ///
    /// The uri can be a file path or web url.
    pub fn open(uri: &str) {
        command(&format!("playerctl open {uri}"));
    }

    /// Get the current loop status.
    pub fn loop_get() -> LoopStatus {
        command("playerctl loop").into()
    }

    /// Set the loop status.
    pub fn loop_set(status: LoopStatus) {
        command(&format!("playerctl loop {:?}", status));
    }

    /// Get the current shuffle status.
    pub fn shuffle_get() -> ShuffleStatus {
        command("playerctl shuffle").into()
    }

    /// Set the shuffle status.
    pub fn shuffle_set(status: ShuffleStatus) {
        command(&format!("playerctl shuffle {:?}", status));
    }
}
