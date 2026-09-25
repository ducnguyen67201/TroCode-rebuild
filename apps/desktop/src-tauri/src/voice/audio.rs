#[cfg(feature = "desktop")]
mod native {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{SampleFormat, StreamConfig};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use tokio::sync::mpsc;

    pub const AUDIO_QUEUE_CAPACITY: usize = 32;

    pub struct MicrophoneCapture {
        stop: std::sync::mpsc::Sender<()>,
        thread: Option<std::thread::JoinHandle<()>>,
        pub sample_rate: u32,
        pub samples: mpsc::Receiver<Vec<i16>>,
        pub failed: Arc<AtomicBool>,
    }

    impl MicrophoneCapture {
        pub fn start() -> Result<Self, &'static str> {
            type Started = Result<(u32, mpsc::Receiver<Vec<i16>>, Arc<AtomicBool>), &'static str>;
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
                        let failed = Arc::new(AtomicBool::new(false));
                        let callback_failed = failed.clone();
                        let error_failed = failed.clone();
                        let error = move |_| {
                            error_failed.store(true, Ordering::Release);
                        };
                        let stream = match supported.sample_format() {
                            SampleFormat::F32 => device.build_input_stream(
                                config,
                                move |data: &[f32], _| {
                                    send_mono(data, channels, &sender, &callback_failed, |value| {
                                        (value.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16
                                    })
                                },
                                error,
                                None,
                            ),
                            SampleFormat::I16 => device.build_input_stream(
                                config,
                                move |data: &[i16], _| {
                                    send_mono(data, channels, &sender, &callback_failed, |value| {
                                        value
                                    })
                                },
                                error,
                                None,
                            ),
                            SampleFormat::U16 => device.build_input_stream(
                                config,
                                move |data: &[u16], _| {
                                    send_mono(data, channels, &sender, &callback_failed, |value| {
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
                        Ok((stream, sample_rate, samples, failed))
                    };
                    match start() {
                        Ok((stream, sample_rate, samples, failed)) => {
                            if started_sender
                                .send(Ok((sample_rate, samples, failed)))
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
            let (sample_rate, samples, failed) = started_receiver
                .recv_timeout(Duration::from_secs(5))
                .map_err(|_| "Microphone capture could not start.")??;
            Ok(Self {
                stop,
                thread: Some(thread),
                sample_rate,
                samples,
                failed,
            })
        }
    }

    impl Drop for MicrophoneCapture {
        fn drop(&mut self) {
            let _ = self.stop.send(());
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    fn send_mono<T: Copy>(
        data: &[T],
        channels: usize,
        sender: &mpsc::Sender<Vec<i16>>,
        failed: &AtomicBool,
        convert: impl Fn(T) -> i16,
    ) {
        if failed.load(Ordering::Acquire) || channels == 0 {
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
        if sender.try_send(mono).is_err() {
            failed.store(true, Ordering::Release);
        }
    }
}

#[cfg(feature = "desktop")]
pub use native::MicrophoneCapture;
