# hardclipper

**hardclipper** is a simple hard clipper _(shocker)_ VST3 and CLAP plugin. Since the purpose of this plugin is to hard clip, it does not oversample (and honestly, I'm a newb at DSP and I don't know how). 

## Parameters

* Input controls the gain of the audio going in.
* Ceiling sets the clipping level. Anything above the celing will get clipped.
* Reduce lowers the level of the clipped portions of the audio. At 0%, everything that's clipped by the ceiling will be set to 0.0. This makes the clipper more distorted and harsh. 
* Output controls the gain of the audio going out. 
* Delta allows to only listen to the portions of the audio that are being clipped. Since the plugin does not have a UI, this is great way to hear what the clipper is doing, sort of as a replacement for showing a clipped waveform on an oscilloscope. 

## Building

After installing [Rust](https://rustup.rs/), you can compile hardclipper as follows:

```shell
cargo xtask bundle hardclipper --release
```
