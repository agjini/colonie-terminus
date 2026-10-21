use bevy::audio::PlaybackMode::{Despawn, Loop};
use bevy::audio::Volume;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::prelude::IndexedRandom;
use ron_asset_manager::Shandle;
use serde::Deserialize;

pub fn plugin(app: &mut App) {
    app.init_resource::<AudioSettings>();
    app.init_resource::<MusicLowered>();
    app.add_systems(Update, (apply_volume, play_tracks));
}

#[derive(Component, Reflect)]
pub struct Music;

#[derive(Component, Reflect)]
pub struct SoundFx;

#[derive(Resource, Default)]
pub struct MusicLowered(pub bool);

const LOWERED_VOLUME_FACTOR: f32 = 0.3;

fn track_volume(audio_settings: &AudioSettings, lowered: &MusicLowered) -> Volume {
    let factor = if lowered.0 {
        LOWERED_VOLUME_FACTOR
    } else {
        1.0
    };
    Volume::Linear(audio_settings.music_volume * factor)
}

fn apply_volume(
    audio_settings: Res<AudioSettings>,
    lowered: Res<MusicLowered>,
    musics: Query<(&mut AudioSink, Has<Track>), With<Music>>,
) {
    if !audio_settings.is_changed() && !lowered.is_changed() {
        return;
    }

    for (mut music, is_track) in musics {
        let volume = if is_track {
            track_volume(&audio_settings, &lowered)
        } else {
            Volume::Linear(audio_settings.music_volume)
        };
        music.set_volume(volume);
    }
}

fn play_tracks(
    mut commands: Commands,
    mut tracks: Query<(Entity, &mut Track), Without<AudioPlayer>>,
    audio_settings: Res<AudioSettings>,
    lowered: Res<MusicLowered>,
) {
    for (entity, mut track) in &mut tracks {
        let Some(source) = track.next_music() else {
            continue;
        };
        commands.entity(entity).insert((
            AudioPlayer::new(source),
            PlaybackSettings {
                volume: track_volume(&audio_settings, &lowered),
                ..PlaybackSettings::REMOVE
            },
        ));
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Reflect, Debug, Deserialize)]
pub enum TrackId {
    Quiet,
    Intense,
}

pub type Tracks = HashMap<TrackId, Vec<Shandle<AudioSource>>>;

#[derive(Component)]
pub struct Track {
    tracks: Tracks,
    current_track: TrackId,
    current_music: Option<usize>,
}

impl Track {
    pub fn new(tracks: Tracks) -> Self {
        Self {
            tracks,
            current_track: TrackId::Quiet,
            current_music: None,
        }
    }

    fn next_music(&mut self) -> Option<Handle<AudioSource>> {
        let musics = self.tracks.get(&self.current_track)?;
        let candidates: Vec<usize> = (0..musics.len())
            .filter(|&i| musics.len() == 1 || Some(i) != self.current_music)
            .collect();
        let index = *candidates.choose(&mut rand::rng())?;
        self.current_music = Some(index);
        Some(musics[index].handle.clone())
    }
}

pub fn track(tracks: Tracks) -> impl Bundle {
    (Track::new(tracks), Music)
}

pub fn sound_fx(handle: Handle<AudioSource>, audio_settings: &AudioSettings) -> impl Bundle {
    (
        AudioPlayer::new(handle),
        SoundFx,
        PlaybackSettings {
            mode: Despawn,
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
            music_volume: 0.1,
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

pub fn lower_music_volume(_: On<PointerClick>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.lower_volume();
}

pub fn raise_music_volume(_: On<PointerClick>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.raise_volume();
}

pub fn lower_sfx_volume(_: On<PointerClick>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.lower_sound_fx_volume();
}

pub fn raise_sfx_volume(_: On<PointerClick>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.raise_sound_fx_volume();
}
