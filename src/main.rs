#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // use eframe::{egui_wgpu, wgpu};

    use std::sync::Arc;

    use eframe::{egui_wgpu, wgpu};

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        wgpu_options: egui_wgpu::WgpuConfiguration {
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: Some(1),
            on_surface_error: Arc::new(|e| {
                println!("WGPU error: {e:?}");
                egui_wgpu::SurfaceErrorAction::SkipFrame
            }),
            wgpu_setup: egui_wgpu::WgpuSetup::CreateNew {
                supported_backends: wgpu::Backends::all(),
                power_preference: wgpu::PowerPreference::LowPower,
                device_descriptor: Arc::new(|_adapter| wgpu::DeviceDescriptor {
                    label: Some("egui-wgpu"),
                    ..Default::default()
                }),
            },
        },
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                // eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                //     .expect("Failed to load icon"),
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(eframe_template::TemplateApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            println!("start_result: {:?}", start_result);
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    // let e_str = e;
                    // let error = format!("<p> {} </p>", e);
                    // loading_text.set_inner_html(&error);
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
