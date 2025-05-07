/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use rodio::{cpal::FromSample, OutputStream, Sample, Sink, Source};

/// Audio player.
pub struct AudioPlayer {
    _s: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    /// Create audio player.
    pub fn new() -> Self {
        let (o, h) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&h).unwrap();
        Self { _s: o, sink }
    }

    /// Play.
    pub fn play(&self) {
        self.sink.play();
    }

    /// Pause.
    pub fn pause(&self) {
        self.sink.pause();
    }

    /// Stop.
    pub fn stop(&self) {
        self.sink.stop();
    }

    /// Clear.
    pub fn clear(&self) {
        self.sink.clear();
    }

    /// Append.
    pub fn append<S>(&self, s: S)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
    {
        self.sink.append(s)
    }

    /// Set volume.
    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    /// Check if playing.
    pub fn is_playing(&self) -> bool {
        !self.sink.empty()
    }
}

unsafe impl Send for AudioPlayer {}
unsafe impl Sync for AudioPlayer {}
