extern crate ffmpeg;
extern crate openal;

use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::path::Path;
use std::env;

enum Decoder {
	Frame(ffmpeg::frame::Audio),
	Error(ffmpeg::Error),
	End(SyncSender<Decoder>),
}

fn decoder<T: AsRef<Path>>(path: T) -> Result<Receiver<Decoder>, ffmpeg::Error> {
	// create a synchronous channel, we don't want to read more than we can play
	let (sender, receiver) = sync_channel(8);

	// open the file with ffmpeg
	let mut format = try!(ffmpeg::format::input(&path));

	// get the audio stream and the decoder for it
	let (mut codec, stream) = {
		let stream = format.streams().find(|s| s.codec().medium() == ffmpeg::media::Type::Audio).expect("no audio stream in the file");
		let codec  = stream.codec().decoder().audio().expect("no audio stream");

		(codec, stream.index())
	};

	thread::spawn(move || {
		// create a resampler to convert the stream to something usable by OpenAL
		let mut resampler = codec.resampler(ffmpeg::format::Sample::I16(ffmpeg::format::sample::Type::Packed), ffmpeg::channel_layout::STEREO, 44100).unwrap();

		// allocate a frame for the decoder
		let mut decoded = ffmpeg::frame::Audio::empty();

		// iterate over the packets in the file
		for (s, packet) in format.packets() {
			// check that the stream of the packet is the audio stream we picked early on
			if s.index() != stream {
				continue;
			}

			// decode the packet
			match codec.decode(&packet, &mut decoded) {
				// if we have decoded a full frame
				Ok(true) => {
					// allocate a frame for the resampling
					let mut resampled = ffmpeg::frame::Audio::empty();

					// clone the attributes from the decoded frame
					resampled.clone_from(&decoded);

					// resample the frame
					resampler.run(&decoded, &mut resampled).unwrap();

					// send it to the receiver
					sender.send(Decoder::Frame(resampled)).unwrap();
				},

				// we didn't get a full frame
				Ok(false) =>
					(),

				// if there was an error in the decoding
				Err(error) =>
					// send it to the receiver
					sender.send(Decoder::Error(error)).unwrap()
			}
		}

		// tell the receiver no more frames will be sent
		sender.send(Decoder::End(sender.clone())).unwrap();
	});

	Ok(receiver)
}

fn main() {
	ffmpeg::init().unwrap();

	// get the channel to receive frames from
	let receiver = decoder(&env::args().nth(1).expect("missing argument")).unwrap();

	// open the default audio device
	let listener = openal::listener::default(&Default::default()).unwrap();

	// create a streaming source
	let mut source = listener.source().unwrap().stream();

	// start our receive loop
	loop {
		// receive a frame
		match receiver.recv() {
			// if we got a frame
			Ok(Decoder::Frame(frame)) => {
				// queue a buffer with the current frame data
				source.push(frame.channels(), frame.plane::<i16>(0), frame.rate()).unwrap();

				// play the source unless it's already playing
				if source.state() != openal::source::State::Playing {
					source.play();
				}

				// sleep for the duration of the frame
				ffmpeg::time::sleep(((1_000.0 / frame.rate() as f32) * frame.samples() as f32 * 1_000.0 - 1_000.0) as u32).unwrap();
			},

			// if we get an error from the decoder
			Ok(Decoder::Error(error)) =>
				// print the error, it's most likely not a fatal error, so we don't
				// break out of the loop
				println!("error: {}", error),

			// if there are no more frames, break out of the loop
			Ok(Decoder::End(..)) =>
				break,

			// if we get an error from the channel
			Err(error) => {
				// print the error
				println!("error: {}", error);

				// and break out, since it's most likely fatal
				break;
			}
		}
	}

	// wait for the source to finish playing
	while source.state() == openal::source::State::Playing {
		ffmpeg::time::sleep(1_000_000).unwrap();
	}
}
