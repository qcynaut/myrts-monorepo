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

use anyhow::{bail, Result};
use byteorder::WriteBytesExt;
use rodio::Source;
use std::{
    io::{Cursor, Write},
    mem::ManuallyDrop,
};

/// native c library for audio encoding and decoding
pub mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(improper_ctypes)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Create opus header
pub fn opus_head() -> Result<*mut c::ogg_packet> {
    let mut writer = Cursor::new(vec![]);
    writer.write_all(b"OpusHead")?;
    writer.write_u8(1)?;
    writer.write_u8(2)?;
    writer.write_u16::<byteorder::LittleEndian>(0)?;
    writer.write_u32::<byteorder::LittleEndian>(48000)?;
    writer.write_u16::<byteorder::LittleEndian>(0)?;
    writer.write_u8(0)?;

    unsafe {
        let op: *mut c::ogg_packet =
            libc::malloc(std::mem::size_of::<c::ogg_packet>()) as *mut c::ogg_packet;
        let mut packet = ManuallyDrop::new(writer.into_inner());
        (*op).packet = packet.as_mut_ptr() as *mut u8;
        (*op).bytes = packet.len() as i64;
        (*op).b_o_s = 1;
        (*op).e_o_s = 0;
        (*op).granulepos = 0;
        (*op).packetno = 0;
        Ok(op)
    }
}

/// Create opus tags
pub fn opus_tags() -> Result<*mut c::ogg_packet> {
    let mut writer = Cursor::new(Vec::new());
    let vendor = b"MyRts";
    writer.write_all(b"OpusTags")?;
    writer.write_u32::<byteorder::LittleEndian>(vendor.len() as u32)?;
    writer.write_all(vendor)?;
    writer.write_u32::<byteorder::LittleEndian>(0)?;

    unsafe {
        let op: *mut c::ogg_packet =
            libc::malloc(std::mem::size_of::<c::ogg_packet>()) as *mut c::ogg_packet;
        let mut packet = ManuallyDrop::new(writer.into_inner());
        (*op).packet = packet.as_mut_ptr() as *mut u8;
        (*op).bytes = packet.len() as i64;
        (*op).b_o_s = 0;
        (*op).e_o_s = 0;
        (*op).granulepos = 0;
        (*op).packetno = 1;
        Ok(op)
    }
}

/// Capsulate rtp packet
pub fn packetize(packet: &[u8], granule_pos: i64, packetno: i64) -> *mut c::ogg_packet {
    unsafe {
        let op: *mut c::ogg_packet =
            libc::malloc(std::mem::size_of::<c::ogg_packet>()) as *mut c::ogg_packet;
        let mut packet = ManuallyDrop::new(packet.to_vec());
        (*op).bytes = packet.len() as i64;
        (*op).packet = packet.as_mut_ptr() as *mut u8;
        (*op).b_o_s = 0;
        (*op).e_o_s = 0;
        (*op).granulepos = granule_pos;
        (*op).packetno = packetno;
        op
    }
}

/// Sample
/// Represents an pcm sample
pub struct Sample {
    data: Vec<i16>,
    pos: usize,
}

impl Sample {
    /// new
    /// Create a new sample by the given data.
    fn new(data: &[i16]) -> Self {
        Self {
            data: data.to_vec(),
            pos: 0,
        }
    }
}

impl Iterator for Sample {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.data.len() {
            let ret = self.data[self.pos];
            self.pos += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl Source for Sample {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
/// Decoder
/// Contains decoder related objects
pub struct Decoder {
    os: *mut c::ogg_stream_state,
    data: Vec<u8>,
    count: usize,
    granule_pos: i64,
}

impl Decoder {
    /// new
    /// Create a new decoder object.
    pub fn new() -> Result<Self> {
        unsafe {
            let os: *mut c::ogg_stream_state =
                libc::malloc(std::mem::size_of::<c::ogg_stream_state>())
                    as *mut c::ogg_stream_state;
            let ret = c::ogg_stream_init(os, 0);
            if ret != 0 {
                libc::free(os as *mut libc::c_void);
                bail!("Cannot initialize ogg_stream_state");
            }

            let mut decoder = Decoder {
                os,
                data: Vec::new(),
                count: 2,
                granule_pos: 0,
            };

            decoder.prepare()?;
            Ok(decoder)
        }
    }

    /// prepare
    /// Prepare the decoder object.
    fn prepare(&mut self) -> Result<()> {
        let head = opus_head()?;
        let tags = opus_tags()?;

        unsafe {
            c::ogg_stream_packetin(self.os, head);
            c::ogg_stream_packetin(self.os, tags);
            libc::free(head as *mut libc::c_void);
            libc::free(tags as *mut libc::c_void);
        }
        self.flush();
        Ok(())
    }

    /// flush
    /// Flush available audio data to the audio object.
    fn flush(&mut self) {
        unsafe {
            loop {
                let page: *mut c::ogg_page =
                    libc::malloc(std::mem::size_of::<c::ogg_page>()) as *mut c::ogg_page;
                let ret = c::ogg_stream_flush(self.os, page);
                if ret == 0 {
                    libc::free(page as *mut libc::c_void);
                    break;
                }
                let head = std::slice::from_raw_parts((*page).header, (*page).header_len as usize);
                self.data.extend_from_slice(head);
                let body = std::slice::from_raw_parts((*page).body, (*page).body_len as usize);
                self.data.extend_from_slice(body);
                libc::free(page as *mut libc::c_void);
            }
        }
    }

    /// write
    /// Write available audio data to the audio object.
    fn write(&mut self) {
        unsafe {
            loop {
                let page: *mut c::ogg_page =
                    libc::malloc(std::mem::size_of::<c::ogg_page>()) as *mut c::ogg_page;
                let ret = c::ogg_stream_pageout(self.os, page);
                if ret == 0 {
                    libc::free(page as *mut libc::c_void);
                    break;
                }
                let head = std::slice::from_raw_parts((*page).header, (*page).header_len as usize);
                self.data.extend_from_slice(head);
                let body = std::slice::from_raw_parts((*page).body, (*page).body_len as usize);
                self.data.extend_from_slice(body);
                libc::free(page as *mut libc::c_void);
            }
        }
    }

    /// reset
    /// Reset the audio object.
    pub fn reset(&mut self) {
        unsafe {
            // libc::free(self.os as *mut libc::c_void);
            let os: *mut c::ogg_stream_state =
                libc::malloc(std::mem::size_of::<c::ogg_stream_state>())
                    as *mut c::ogg_stream_state;
            let ret = c::ogg_stream_init(os, 0);
            if ret != 0 {
                libc::free(os as *mut libc::c_void);
                return;
            }
            libc::free(self.os as *mut libc::c_void);
            self.os = os;
            self.count = 2;
            self.granule_pos = 0;
            self.data.clear();

            let _ = self.prepare();
        }
    }

    /// decode
    /// Decode the packet
    pub fn decode(&mut self, packet: &[u8]) -> Vec<Sample> {
        unsafe {
            let sample_count = c::opus_packet_get_nb_samples(
                packet.as_ptr() as *const u8,
                packet.len() as i32,
                48000,
            );
            if sample_count > 0 {
                self.granule_pos += sample_count as i64;
            }
            let op = packetize(packet, self.granule_pos, self.count as i64);
            self.count += 1;
            c::ogg_stream_packetin(self.os, op);
            libc::free(op as *mut libc::c_void);
            self.write();

            let mut result = vec![];

            if self.count > 20 {
                self.flush();
                let mut err = 0;
                let op =
                    c::op_open_memory(self.data.as_ptr() as *const u8, self.data.len(), &mut err);
                if err == 0 {
                    loop {
                        let mut pcm = [0i16; 11520];
                        let ret =
                            c::op_read_stereo(op, pcm.as_mut_ptr() as *mut i16, pcm.len() as i32);
                        if ret <= 0 {
                            break;
                        }
                        let sample = Sample::new(&pcm[..(ret * 2) as usize]);
                        result.push(sample);
                    }
                }
                libc::free(op as *mut libc::c_void);
                self.reset();
            }

            result
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.os as *mut libc::c_void);
        }
    }
}

unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}
