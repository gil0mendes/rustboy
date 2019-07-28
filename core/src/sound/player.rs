use cpal;
use std;
use crate::sound;

pub struct CpalPlayer {
    voice: cpal::Voice
}

impl CpalPlayer {
    pub fn get() -> Option<CpalPlayer> {
        // try get endpoints
        if cpal::get_endpoints_list().count() == 0 { return None }

        // get the default endpoint
        let endpoint = match cpal::get_default_endpoint() {
            Some(e) => e,
            None => return None,
        };

        let mut wanted_samplerate = None;
        let mut wanted_sampleformat = None;

        // iterate through the supported formats
        for format in endpoint.get_supported_formats_list().unwrap() {
            // get the wanted sample rate
            match wanted_samplerate {
                None => wanted_samplerate = Some(format.samples_rate),
                Some(cpal::SamplesRate(r)) if r < format.samples_rate.0 && r < 192000 => wanted_samplerate = Some(format.samples_rate),
                _ => {}
            }

            // get the wanted sample format
            match wanted_sampleformat {
                None => wanted_sampleformat = Some(format.data_type),
                Some(cpal::SampleFormat::F32) => {}
                Some(_) if format.data_type == cpal::SampleFormat::F32 => wanted_sampleformat = Some(format.data_type),
                _ => {}
            }
        }

        // check if the wanted sample rate and format are found
        if wanted_samplerate.is_none() || wanted_sampleformat.is_none() {
            return None
        }

        // instance a new format
        let format = cpal::Format {
            channels: vec![cpal::ChannelPosition::FrontLeft, cpal::ChannelPosition::FrontRight],
            samples_rate: wanted_samplerate.unwrap(),
            data_type: wanted_sampleformat.unwrap(),
        };

        match cpal::Voice::new(&endpoint, &format) {
            Ok(v) => Some(CpalPlayer { voice: v }),
            Err(_) => None,
        }
    }
}

impl sound::AudioPlayer for CpalPlayer {
    fn play(&mut self, buf_left: &[f32], buf_right: &[f32]) {
        let left_idx = self.voice.format().channels.iter().position(|c| *c == cpal::ChannelPosition::FrontLeft);
        let right_idx = self.voice.format().channels.iter().position(|c| *c == cpal::ChannelPosition::FrontRight);

        // get the number of channels
        let channel_count = self.voice.format().channels.len();

        let count = buf_left.len();
        let mut done = 0;
        let mut lastdone = count;

        while lastdone != done && done < count {
            lastdone = done;
            let buf_left_next = &buf_left[done..];
            let buf_right_next = &buf_right[done..];

            match self.voice.append_data(count - done) {
                cpal::UnknownTypeBuffer::U16(mut buffer) => {
                    for (i, sample) in buffer.chunks_mut(channel_count).enumerate() {
                        if sample.len() < channel_count {
                            break;
                        }

                        if let Some(idx) = left_idx {
                            sample[idx] = (buf_left_next[i] * (std::i16::MAX as f32) + (std::i16::MAX as f32)) as u16;
                        }

                        if let Some(idx) = right_idx {
                            sample[idx] = (buf_right_next[i] * (std::i16::MAX as f32) + (std::i16::MAX as f32)) as u16;
                        }

                        done += 1;
                    }
                }

                cpal::UnknownTypeBuffer::I16(mut buffer) => {
                    for (i, sample) in buffer.chunks_mut(channel_count).enumerate() {
                        if sample.len() < channel_count {
                            break;
                        }

                        if let Some(idx) = left_idx {
                            sample[idx] = (buf_left_next[i] * std::i16::MAX as f32) as i16;
                        }

                        if let Some(idx) = right_idx {
                            sample[idx] = (buf_right_next[i] * std::i16::MAX as f32) as i16;
                        }

                        done += 1;
                    }
                }

                cpal::UnknownTypeBuffer::F32(mut buffer) => {
                    for (i, sample) in buffer.chunks_mut(channel_count).enumerate() {
                        if sample.len() < channel_count {
                            break;
                        }

                        if let Some(idx) = left_idx {
                            sample[idx] = buf_left_next[i];
                        }

                        if let Some(idx) = right_idx {
                            sample[idx] = buf_right_next[i];
                        }

                        done += 1;
                    }
                }
            }
        }

        // play the sample
        self.voice.play();
    }

    fn samples_rate(&self) -> u32 {
        self.voice.format().samples_rate.0
    }

    fn underflowed(&self) -> bool {
        self.voice.underflowed()
    }
}
