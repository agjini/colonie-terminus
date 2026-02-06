use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default());
    app.add_systems(Startup, set_default_volume);
}

pub fn music(handle: Handle<AudioSample>) -> impl Bundle {
    (MusicPool, SamplePlayer::new(handle).looping())
}

pub fn sound_effect(handle: Handle<AudioSample>) -> impl Bundle {
    SamplePlayer::new(handle)
}

pub const CONVERTER: PerceptualVolume = PerceptualVolume::new();

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

const STEP: f32 = 0.1;

pub fn lower_music_volume(
    _: On<Pointer<Click>>,
    mut music: Single<&mut VolumeNode, With<SamplerPool<MusicPool>>>,
) {
    music.volume = decrement_volume(music.volume);
}

pub fn raise_music_volume(
    _: On<Pointer<Click>>,
    mut music: Single<&mut VolumeNode, With<SamplerPool<MusicPool>>>,
) {
    music.volume = increment_volume(music.volume);
}

pub fn lower_sfx_volume(
    _: On<Pointer<Click>>,
    mut sfx: Single<&mut VolumeNode, With<SoundEffectsBus>>,
) {
    sfx.volume = decrement_volume(sfx.volume);
}

pub fn raise_sfx_volume(
    _: On<Pointer<Click>>,
    mut sfx: Single<&mut VolumeNode, With<SoundEffectsBus>>,
) {
    sfx.volume = increment_volume(sfx.volume);
}

fn increment_volume(volume: Volume) -> Volume {
    let perceptual = CONVERTER.volume_to_perceptual(volume);
    let new_perceptual = (perceptual + STEP).min(MAX_VOLUME);
    CONVERTER.perceptual_to_volume(new_perceptual)
}

fn decrement_volume(volume: Volume) -> Volume {
    let perceptual = CONVERTER.volume_to_perceptual(volume);
    let new_perceptual = (perceptual - STEP).max(MIN_VOLUME);
    CONVERTER.perceptual_to_volume(new_perceptual)
}

fn set_default_volume(
    mut master: Single<&mut VolumeNode, (With<MainBus>, Without<SoundEffectsBus>)>,
    mut sfx: Single<&mut VolumeNode, (With<SoundEffectsBus>, Without<MainBus>)>,
) {
    master.volume = CONVERTER.perceptual_to_volume(0.7);
    sfx.volume = CONVERTER.perceptual_to_volume(1.2);
}
