use dinai::window::WindowConfig;

fn main() -> Result<(), String> {
    let conf = WindowConfig {
        title: "dinai",
        width: 1280,
        height: 720,
    };

    dinai::run(conf)
}
