use bevy::prelude::{App, Plugin};

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        use bevy::prelude::Update;

        app.add_systems(Update, handle_browser_resize);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, _app: &mut App) {}
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(
    mut primary_query: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    for mut window in &mut primary_query {
        let wasm_window = web_sys::window().unwrap();
        let (target_width, target_height) = (
            wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
            wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
        );
        if window.resolution.width() != target_width || window.resolution.height() != target_height
        {
            window.resolution.set(target_width, target_height);
        }
    }
}
