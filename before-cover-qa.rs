use fastpotify::{app::{App, AppOptions}, backend::Waker, model::{Dialog, Page}, paths::AppDirs, settings::Settings};
struct Capture { app: App, path: String, frames: u32 }
impl eframe::App for Capture {
    fn logic(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.app.background_frame(ctx);
        self.frames += 1;
        ctx.request_repaint();
        if self.frames == 25 { ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default())); }
        let shot = ctx.input(|i| i.events.iter().find_map(|e| if let egui::Event::Screenshot { image, .. } = e { Some(image.clone()) } else { None }));
        if let Some(shot) = shot {
            eprintln!("Captured {}x{}: {}", shot.size[0], shot.size[1], self.path);
            let pixels = shot.pixels.iter().flat_map(|p| p.to_srgba_unmultiplied()).collect();
            image::RgbaImage::from_raw(shot.size[0] as u32, shot.size[1] as u32, pixels).unwrap().save(&self.path).unwrap();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) { self.app.frame_ui(ui); }
}
fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let width: f32 = args[2].parse().unwrap(); let height: f32 = args[3].parse().unwrap();
    let waker = Waker::default();
    let root = std::env::temp_dir().join(format!("fastpotify-cover-qa-{}", std::process::id()));
    let mut app = App::new(&waker, AppDirs { config: root.join("config"), state: root.join("state"), cache: root.join("cache") }, Settings::default(), AppOptions { media_controls: false, tray: false });
    fastpotify::demo::populate(&mut app);
    app.open(Page::Playlist("pl1".into()));
    fastpotify::demo::apply_flags(&mut app, None, Some(&args[4]));
    app.dialog = Some(Dialog::EditPlaylist { id: "pl1".into(), name: "Long Way Home".into(), description: "Songs for the road".into(), public: false });
    let path = args[1].clone();
    eframe::run_native("Cover QA", eframe::NativeOptions { persist_window: false, persistence_path: Some(root.join("eframe")), viewport: egui::ViewportBuilder::default().with_inner_size([width, height]), ..Default::default() }, Box::new(move |cc| {
        waker.attach(&cc.egui_ctx); app.attach(&cc.egui_ctx);
        Ok(Box::new(Capture { app, path, frames: 0 }))
    }))
}
