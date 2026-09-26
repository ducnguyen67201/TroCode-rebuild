#[cfg(feature = "desktop")]
mod native {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{SampleFormat, StreamConfig};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::Duration;
    use tokio::sync::mpsc;

    pub const AUDIO_QUEUE_CAPACITY: usize = 32;
    const CAPTURE_OK: u8 = 0;
    const STREAM_FAILED: u8 = 1;
    const BUFFER_OVERFLOWED: u8 = 2;

    pub struct MicrophoneCapture {
        stop: std::sync::mpsc::Sender<()>,
        thread: Option<std::thread::JoinHandle<()>>,
        pub sample_rate: u32,
        pub samples: mpsc::Receiver<Vec<i16>>,
        failure: Arc<AtomicU8>,
    }

    impl MicrophoneCapture {
        pub fn start() -> Result<Self, &'static str> {
            type Started = Result<(u32, mpsc::Receiver<Vec<i16>>, Arc<AtomicU8>), &'static str>;
            let (started_sender, started_receiver) = std::sync::mpsc::sync_channel::<Started>(1);
            let (stop, stop_receiver) = std::sync::mpsc::channel();
            let thread = std::thread::Builder::new()
                .name("tro-microphone-capture".into())
                .spawn(move || {
                    let start = || -> Result<_, &'static str> {
                        let device = cpal::default_host()
                            .default_input_device()
                            .ok_or("No microphone is available.")?;
                        let supported = device
                            .default_input_config()
                            .map_err(|_| "Microphone configuration is unavailable.")?;
                        let sample_rate = supported.sample_rate();
                        let config: StreamConfig = supported.into();
                        let channels = usize::from(config.channels);
                        let (sender, samples) = mpsc::channel(AUDIO_QUEUE_CAPACITY);
                        let failure = Arc::new(AtomicU8::new(CAPTURE_OK));
                        let callback_failure = failure.clone();
                        let stream_failure = failure.clone();
                        let error = move |error| {
                            if stream_failure
                                .compare_exchange(
                                    CAPTURE_OK,
                                    STREAM_FAILED,
                                    Ordering::AcqRel,
                                    Ordering::Acquire,
                                )
                                .is_ok()
                            {
                                #[cfg(debug_assertions)]
                                eprintln!("tro diagnostic: microphone_stream_failed error={error}");
                            }
                        };
                        let stream = match supported.sample_format() {
                            SampleFormat::F32 => device.build_input_stream(
                                config,
                                move |data: &[f32], _| {
                                    send_mono(data, channels, &sender, &callback_failure, |value| {
                                        (value.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16
                                    })
                                },
                                error,
                                None,
                            ),
                            SampleFormat::I16 => device.build_input_stream(
                                config,
                                move |data: &[i16], _| {
                                    send_mono(data, channels, &sender, &callback_failure, |value| {
                                        value
                                    })
                                },
                                error,
                                None,
                            ),
                            SampleFormat::U16 => device.build_input_stream(
                                config,
                                move |data: &[u16], _| {
                                    send_mono(data, channels, &sender, &callback_failure, |value| {
                                        (i32::from(value) - 32_768) as i16
                                    })
                                },
                                error,
                                None,
                            ),
                            _ => return Err("Unsupported microphone sample format."),
                        }
                        .map_err(|_| "Microphone capture could not start.")?;
                        stream
                            .play()
                            .map_err(|_| "Microphone capture could not start.")?;
                        Ok((stream, sample_rate, samples, failure))
                    };
                    match start() {
                        Ok((stream, sample_rate, samples, failure)) => {
                            if started_sender
                                .send(Ok((sample_rate, samples, failure)))
                                .is_ok()
                            {
                                let _ = stop_receiver.recv();
                            }
                            drop(stream);
                        }
                        Err(error) => {
                            let _ = started_sender.send(Err(error));
                        }
                    }
                })
                .map_err(|_| "Microphone capture could not start.")?;
            let (sample_rate, samples, failure) = started_receiver
                .recv_timeout(Duration::from_secs(5))
                .map_err(|_| "Microphone capture could not start.")??;
            Ok(Self {
                stop,
                thread: Some(thread),
                sample_rate,
                samples,
                failure,
            })
        }

        pub fn stop(&mut self) {
            let _ = self.stop.send(());
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }

        pub fn failure_message(&self) -> Option<&'static str> {
            match self.failure.load(Ordering::Acquire) {
                STREAM_FAILED => Some("Microphone stream stopped."),
                BUFFER_OVERFLOWED => Some("Microphone audio buffer overflowed."),
                _ => None,
            }
        }
    }

    impl Drop for MicrophoneCapture {
        fn drop(&mut self) {
            self.stop();
        }
    }

    fn send_mono<T: Copy>(
        data: &[T],
        channels: usize,
        sender: &mpsc::Sender<Vec<i16>>,
        failure: &AtomicU8,
        convert: impl Fn(T) -> i16,
    ) {
        if failure.load(Ordering::Acquire) != CAPTURE_OK || channels == 0 {
            return;
        }
        let mut mono = Vec::with_capacity(data.len() / channels);
        for frame in data.chunks_exact(channels) {
            let total = frame
                .iter()
                .copied()
                .map(&convert)
                .map(i32::from)
                .sum::<i32>();
            mono.push((total / channels as i32) as i16);
        }
        if let Err(mpsc::error::TrySendError::Full(_)) = sender.try_send(mono)
            && failure
                .compare_exchange(
                    CAPTURE_OK,
                    BUFFER_OVERFLOWED,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
        {
            #[cfg(debug_assertions)]
            eprintln!("tro diagnostic: microphone_buffer_overflow capacity={AUDIO_QUEUE_CAPACITY}");
        }
    }
}

#[cfg(feature = "desktop")]
pub use native::MicrophoneCapture;
