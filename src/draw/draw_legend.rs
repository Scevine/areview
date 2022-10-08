use crate::room::Sector;
use crate::LabelColor;
use nannou::prelude::Vec2;
use nannou::Draw;

const LEGEND_SECTORS: &'static [(Sector, &'static str)] = &[
    (Sector::Inside, "Inside"),
    (Sector::House, "House"),
    (Sector::City, "City"),
    (Sector::RogueGuild, "Rogue Guild"),
    (Sector::Field, "Field"),
    (Sector::Hills, "Hills"),
    (Sector::Forest, "Forest"),
    (Sector::Mountain, "Mountain"),
    (Sector::Desert, "Desert"),
    (Sector::WaterSwim, "Water (swim)"),
    (Sector::WaterNoswim, "Water (no swim)"),
    (Sector::Underwater, "Underwater"),
    (Sector::OnBottom, "On Bottom"),
    (Sector::Air, "Air"),
];

pub fn draw_legend(draw: &Draw) {
    const CELL_WIDTH: f32 = 100f32;
    const CELL_HEIGHT: f32 = 20f32;
    for (y, (sector, sector_name)) in LEGEND_SECTORS.iter().enumerate() {
        let xy = Vec2::new(5f32, -5f32 - (y) as f32 * (CELL_HEIGHT + 5f32));
        let LabelColor {
            background,
            foreground,
        } = sector.color();
        let cell_center: Vec2 = Vec2::new(CELL_WIDTH, CELL_HEIGHT * -1f32) * 0.5;
        draw.xy(xy + cell_center)
            .rect()
            .w_h(CELL_WIDTH, CELL_HEIGHT)
            .color(background);
        draw.xy(xy + cell_center + 2f32)
            .text(sector_name)
            .w_h(CELL_WIDTH - 4f32, CELL_HEIGHT - 4f32)
            .left_justify()
            .color(foreground);
    }
}
