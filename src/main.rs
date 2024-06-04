use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::process::exit;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod metronome {
    use super::*;
    #[derive(Clone)]
    pub struct TimeSignature {
        top: u8,
        bottom: u8,
    }

    impl TimeSignature {
        pub fn new(top: u8, bottom: u8) -> Self {
            Self { top, bottom }
        }
    }

    #[derive(Clone)]
    pub struct Metronome {
        bpm: u8,
        signature: TimeSignature,
        sink: Arc<Sink>,
        _stream: Arc<OutputStream>,
    }

    impl Metronome {
        pub fn new(bpm: u8, signature: TimeSignature) -> Self {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
            Self {
                bpm,
                signature,
                sink,
                _stream: Arc::new(stream),
            }
        }

        fn get_duration(self) -> Duration {
            let bpm_as_f64 = self.bpm as f64;
            let note_factor = 4.0f64 / self.signature.bottom as f64;
            let quarter_duration = 60.0f64 / bpm_as_f64;
            let note_duration = quarter_duration * note_factor;
            Duration::from_secs_f64(note_duration)
        }

        fn get_repeats(self) -> u8 {
            self.signature.top
        }

        pub fn run_metronome(self) -> () {
            let duration = self.clone().get_duration();
            let tone_duration = duration / 4 * 3;
            let pause_duration = duration / 4;
            let repeats = self.clone().get_repeats() - 1;

            let low_beep_source = SineWave::new(440.0)
                .take_duration(tone_duration)
                .amplify(0.20);

            let high_beep_source = SineWave::new(660.0)
                .take_duration(tone_duration)
                .amplify(0.20);

            let no_beep_source = SineWave::new(0.00)
                .take_duration(pause_duration)
                .amplify(0.0);

            loop {
                self.sink.append(high_beep_source.clone());
                self.sink.append(no_beep_source.clone());

                for _ in 0..repeats {
                    self.sink.append(low_beep_source.clone());
                    self.sink.append(no_beep_source.clone());
                    // thread::sleep(pause_duration);
                }

                self.sink.sleep_until_end();
            }
        }
    }

    #[derive(Clone)]
    pub struct PolyrythmicMetronome {
        bpm: u8,
        signatures: Vec<TimeSignature>,
        sink: Arc<Sink>,
        _stream: Arc<OutputStream>,
    }

    impl PolyrythmicMetronome {
        pub fn new(bpm: u8, signatures: Vec<TimeSignature>) -> Self {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
            Self {
                bpm,
                signatures,
                sink,
                _stream: Arc::new(stream),
            }
        }

        fn find_common_denominator(signatures: Vec<Signature>) -> u8 {
            let denominators = signatures.iter().map(|x| x.bottom).collect();
        }

        fn get_duration(self, signature: &TimeSignature) -> Duration {
            let bpm_as_f64 = self.bpm as f64;
            let note_factor = 4.0f64 / signature.bottom as f64;
            let quarter_duration = 60.0f64 / bpm_as_f64;
            let note_duration = quarter_duration * note_factor;
            Duration::from_secs_f64(note_duration)
        }

        fn get_durations(self) -> Vec<Duration> {
            self.signatures
                .iter()
                .map(|x| self.clone().get_duration(x))
                .collect()
        }

        fn get_repeats(self) -> Vec<u8> {
            self.signatures.iter().map(|x| x.top).collect()
        }

        // Figure out how to run multiple metronomes over one another
        // Maybe let them make a list at what point they want to start playing a note
        // a (0,      1,      2,      3,      4)
        // b (0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5)
        // Then put the durations in a set
        // Find the difference between two start durations. Play a note for 3/4 of the difference
        // and be silent for 1/4 of the difference
        pub fn run_metronome(self) -> () {}
    }
}

use metronome::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if (args.len() - 2) % 2 != 0 {
        println!("Please input the count, bar length and bpm of a quarter note");
        panic!("Wrong number of arguments");
    }

    let bpm: u8 = args[1].parse().unwrap();
    let signature = TimeSignature::new(args[2].parse().unwrap(), args[3].parse().unwrap());

    let metronome = Metronome::new(bpm, signature);

    metronome.run_metronome();
}
