use bevy::audio::PlaybackMode::Loop;
use bevy::audio::Volume;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<AudioSettings>();
    app.add_systems(Update, apply_volume);
}

#[derive(Component, Reflect)]
pub struct Music;

#[derive(Component, Reflect)]
pub struct SoundFx;

fn apply_volume(audio_settings: Res<AudioSettings>, musics: Query<&mut AudioSink, With<Music>>) {
    if !audio_settings.is_changed() {
        return;
    }

    let vol = Volume::Linear(audio_settings.music_volume);
    for mut music in musics.into_iter() {
        music.set_volume(vol);
    }
}

pub fn music(handle: Handle<AudioSource>, audio_settings: &AudioSettings) -> impl Bundle {
    (
        AudioPlayer::new(handle),
        Music,
        PlaybackSettings {
            mode: Loop,
            volume: Volume::Linear(audio_settings.music_volume),
            ..default()
        },
    )
}

pub fn sound_fx(handle: Handle<AudioSource>, audio_settings: &AudioSettings) -> impl Bundle {
    (
        AudioPlayer::new(handle),
        SoundFx,
        PlaybackSettings {
            volume: Volume::Linear(audio_settings.sound_fx_volume),
            ..default()
        },
    )
}

#[derive(Resource, Reflect)]
pub struct AudioSettings {
    pub music_volume: f32,
    pub sound_fx_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.5,
            sound_fx_volume: 0.5,
        }
    }
}

impl AudioSettings {
    fn raise_volume(&mut self) {
        self.music_volume = (self.music_volume + 0.1).clamp(0.0, 1.0);
    }
    fn lower_volume(&mut self) {
        self.music_volume = (self.music_volume - 0.1).clamp(0.0, 1.0);
    }
    fn raise_sound_fx_volume(&mut self) {
        self.sound_fx_volume = (self.sound_fx_volume + 0.1).clamp(0.0, 1.0);
    }
    fn lower_sound_fx_volume(&mut self) {
        self.sound_fx_volume = (self.sound_fx_volume - 0.1).clamp(0.0, 1.0);
    }
}

pub fn lower_music_volume(_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.lower_volume();
}

pub fn raise_music_volume(_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.raise_volume();
}

pub fn lower_sfx_volume(_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.lower_sound_fx_volume();
}

pub fn raise_sfx_volume(_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.raise_sound_fx_volume();
}
