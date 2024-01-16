use nih_plug::{prelude::*, log::{log, Level}};
use std::sync::Arc;

struct Hardclipper {
    params: Arc<HardclipperParams>,
}

impl Default for Hardclipper {
    fn default() -> Self {
        Self {
            params: Arc::new(HardclipperParams::default()),
        }
    }
}

/// Hardclipper implementation with the actual clipping function.
impl Hardclipper {
    /// Clip the given sample and return the result.
    fn clip(&self, sample: f32) -> f32 {
        let input_gain = self.params.input_gain.smoothed.next();
        let ceiling = self.params.ceiling.smoothed.next();
        let reduce = self.params.reduce.smoothed.next();
        let output_gain = self.params.output_gain.smoothed.next();
        let delta = self.params.delta.value();

        // Adjust the input gain.
        let input = sample * input_gain;
                
        // Clip audio above the threshold.
        let output = if input.abs() > ceiling {
            input.signum() * ceiling * reduce
        } else {
            input
        };

        // Return clipped audio, or the delta if it's on
        output_gain * if delta { 
            sample - output 
        } else { 
            output 
        }
    }
}

#[derive(Params)]
struct HardclipperParams {
    #[id = "input_gain"]
    pub input_gain: FloatParam,
    
    #[id = "ceiling"]
    pub ceiling: FloatParam,
    
    #[id = "reduce"]
    pub reduce: FloatParam,
    
    #[id = "output_gain"]
    pub output_gain: FloatParam,
    
    #[id = "delta"]
    pub delta: BoolParam,
}

impl Default for HardclipperParams {
    fn default() -> Self {
        Self {
            // ---------------------------------------------------------------- Input
            input_gain: FloatParam::new(
                "Input",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            
            // ---------------------------------------------------------------- Celing
            ceiling: FloatParam::new(
                "Ceiling",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            
            // ---------------------------------------------------------------- Reduce
            reduce: FloatParam::new(
                "Reduce",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
            // .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            // ---------------------------------------------------------------- Output
            output_gain: FloatParam::new(
                "Output",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            
            // ---------------------------------------------------------------- Delta
            delta: BoolParam::new(
                "Delta", 
                false
            ),
        }
    }
}

impl Plugin for Hardclipper {
    const NAME: &'static str = "hardclipper";
    const VENDOR: &'static str = "LASHLIGHT";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "lashlight@proton.me";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        log!(Level::Info, "hardclipper initialized.");
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        log!(Level::Info, "hardclipper reset.")
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
                
        let channels = buffer.as_slice();
        
        // For each channel (L, R)
        for i in 0..channels.len() {
            let channel = channels.get_mut(i).unwrap();
        
            // For each element in the channel buffer (i.e. 1024 elements)
            for j in 0..channel.len() {
                let sample = channel.get_mut(j).unwrap();
                *sample = self.clip(*sample);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Hardclipper {
    const CLAP_ID: &'static str = "com.erroreyes.hardclipper";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple hard clipper");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Hardclipper {
    const VST3_CLASS_ID: [u8; 16] = *b"erreyeshardclipp";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(Hardclipper);
nih_export_vst3!(Hardclipper);
