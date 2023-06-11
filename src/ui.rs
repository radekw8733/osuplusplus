use bevy::prelude::{Query, Res, EventWriter};
use bevy_egui::{egui::{self, Align2, Vec2}, EguiContext};

use crate::{map::{OsuBeatmapPacksBriefedLoaded, MapLoadingWanted}};

pub fn display_loader_window(mut egui_ctx: Query<&mut EguiContext>, beatmaps: Res<OsuBeatmapPacksBriefedLoaded>, mut map_event: EventWriter<MapLoadingWanted>) {
    egui::Window::new("Beatmap select")
        .anchor(Align2::RIGHT_TOP, Vec2 { x: -10.0, y: 10.0 })
        .show(egui_ctx.single_mut().get_mut(), |ui| {
            for beatmap_pack in &beatmaps.0 {
                ui.collapsing(format!("{} - {}", beatmap_pack.0[0].artist, beatmap_pack.0[0].title), |ui| {
                    for beatmap in &beatmap_pack.0 {
                        ui.horizontal(|ui| {
                            ui.label(&beatmap.version);
                            if ui.button("Play").clicked() {
                                map_event.send(MapLoadingWanted(beatmap.path.clone()));
                            }
                        });
                    }
                });
            }
        });
}