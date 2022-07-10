use serial_test::serial;
use std::time::Duration;

#[allow(clippy::wildcard_imports)]
use crate::*;

fn sleep() {
    std::thread::sleep(Duration::from_millis(1000));
}

#[test]
fn player_status_from_str() {
    assert_eq!(
        PlayerStatus::from_str("Playing").expect("PlayerStatus from str failed for 'Playing'"),
        PlayerStatus::Playing
    );
    assert_eq!(
        PlayerStatus::from_str("Paused").expect("PlayerStatus from str failed for 'Paused'"),
        PlayerStatus::Paused
    );
    assert_eq!(
        PlayerStatus::from_str("Stopped").expect("PlayerStatus from str failed for 'Stopped'"),
        PlayerStatus::Stopped
    );
    assert_eq!(PlayerStatus::from_str("Unknown"), Err(()));
}

#[test]
fn player_status_to_str() {
    assert_eq!(format!("{:?}", PlayerStatus::Playing), "Playing");
    assert_eq!(format!("{:?}", PlayerStatus::Paused), "Paused");
    assert_eq!(format!("{:?}", PlayerStatus::Stopped), "Stopped");
}

#[test]
fn loop_status_from_str() {
    assert_eq!(
        LoopStatus::from_str("None").expect("LoopStatus from str failed for 'None'"),
        LoopStatus::None
    );
    assert_eq!(
        LoopStatus::from_str("Track").expect("LoopStatus from str failed for 'Track'"),
        LoopStatus::Track
    );
    assert_eq!(
        LoopStatus::from_str("Playlist").expect("LoopStatus from str failed for 'Playlist'"),
        LoopStatus::Playlist
    );
    assert_eq!(LoopStatus::from_str("Unknown"), Err(()));
}

#[test]
fn loop_status_to_str() {
    assert_eq!(format!("{:?}", LoopStatus::None), "None");
    assert_eq!(format!("{:?}", LoopStatus::Track), "Track");
    assert_eq!(format!("{:?}", LoopStatus::Playlist), "Playlist");
}

#[test]
fn shuffle_status_from_str() {
    assert_eq!(
        ShuffleStatus::from_str("On").expect("ShuffleStatus from str failed for 'On'"),
        ShuffleStatus::On
    );
    assert_eq!(
        ShuffleStatus::from_str("Off").expect("ShuffleStatus from str failed for 'Off'"),
        ShuffleStatus::Off
    );
    assert_eq!(
        ShuffleStatus::from_str("Toggle").expect("ShuffleStatus from str failed for 'Toggle'"),
        ShuffleStatus::Toggle
    );
    assert_eq!(ShuffleStatus::from_str("Unknown"), Err(()));
}

#[test]
fn shuffle_status_to_str() {
    assert_eq!(format!("{:?}", ShuffleStatus::On), "On");
    assert_eq!(format!("{:?}", ShuffleStatus::Off), "Off");
    assert_eq!(format!("{:?}", ShuffleStatus::Toggle), "Toggle");
}

#[test]
#[serial]
fn play_pause() {
    PlayerCtl::play_pause();
    sleep();
    PlayerCtl::play_pause();
    sleep();
}

#[test]
#[serial]
fn pause_then_play_status() {
    PlayerCtl::pause();
    sleep();
    assert_eq!(PlayerCtl::status(), PlayerStatus::Paused);
    PlayerCtl::play();
    sleep();
    assert_eq!(PlayerCtl::status(), PlayerStatus::Playing);
}

#[test]
#[serial]
fn next() {
    PlayerCtl::next();
    sleep();
}

#[test]
#[serial]
fn previous() {
    PlayerCtl::previous();
    sleep();
}

#[test]
#[serial]
fn position() {
    PlayerCtl::position(20.);
    sleep();
    PlayerCtl::position(-20.);
    sleep();
}

#[test]
#[serial]
fn volume() {
    // Honestly, I don't know how playerctl's volume control works.
    // I could never get it to work.
    // This test is just to make sure it doesn't crash.

    PlayerCtl::volume(0.3);
    sleep();
    PlayerCtl::volume(1.);
    sleep();
}

#[test]
#[serial]
fn metadata() {
    // Can't test this without knowing the track data beforehand.
    // This test is just to make sure it doesn't crash.

    let _metadata = PlayerCtl::metadata();
}

#[test]
#[serial]
fn open() {
    PlayerCtl::open(
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
    );
}

#[test]
#[serial]
fn loop_test() {
    PlayerCtl::loop_set(LoopStatus::None);
    sleep();
    assert_eq!(PlayerCtl::loop_get(), LoopStatus::None);

    PlayerCtl::loop_set(LoopStatus::Track);
    sleep();
    assert_eq!(PlayerCtl::loop_get(), LoopStatus::Track);

    PlayerCtl::loop_set(LoopStatus::Playlist);
    sleep();
    assert_eq!(PlayerCtl::loop_get(), LoopStatus::Playlist);
}

#[test]
#[serial]
fn shuffle() {
    PlayerCtl::shuffle_set(ShuffleStatus::On);
    sleep();
    assert_eq!(PlayerCtl::shuffle_get(), ShuffleStatus::On);

    PlayerCtl::shuffle_set(ShuffleStatus::Off);
    sleep();
    assert_eq!(PlayerCtl::shuffle_get(), ShuffleStatus::Off);

    PlayerCtl::shuffle_set(ShuffleStatus::Toggle);
    sleep();
    assert_eq!(PlayerCtl::shuffle_get(), ShuffleStatus::On);
}

#[test]
#[serial]
fn stop_status() {
    PlayerCtl::stop();
    assert_eq!(PlayerCtl::status(), PlayerStatus::Stopped);
    sleep();
}
