//! Cycle Audio — device switching for the Streamfog→SonoBus cycle.
//!
//! Two panel actions propose `select_audio_devices` for the BlackHole
//! streaming pair (mic = BlackHole 2ch, speaker = BlackHole 16ch) or back
//! to the system default. Device IDs are the host IDs Serein persists in
//! its preferences (coreaudio UIDs). The empty string is the local
//! "clear to system default" sentinel added to SelectAudioDevices.
use serein_extension_sdk::{
	export, AppAction, AppInvocation, AppOutput, Element, HostEffect, Output,
};

const INPUT_ID: &str = "coreaudio:BlackHole2ch_UID";
const OUTPUT_ID: &str = "coreaudio:BlackHole16ch_UID";

fn handle(input: AppInvocation) -> AppOutput {
	let (description, action) = match input.invocation.action.as_str() {
		"streaming" => (
			"Streaming: microphone = BlackHole 2ch, speaker = BlackHole 16ch.",
			AppAction::SelectAudioDevices {
				input_id: Some(INPUT_ID.into()),
				output_id: Some(OUTPUT_ID.into()),
			},
		),
		"normal" => (
			"Normal: microphone and speaker return to the system default.",
			AppAction::SelectAudioDevices {
				input_id: Some(String::new()),
				output_id: Some(String::new()),
			},
		),
		_ => return AppOutput::default(),
	};
	AppOutput {
		output: Output {
			panel: vec![Element::Text {
				text: description.into(),
			}],
			..Default::default()
		},
		effects: vec![HostEffect::AppAction { action }],
		..Default::default()
	}
}

export!(handle);
