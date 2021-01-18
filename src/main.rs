use orbtk::prelude::*;

fn main() {
    orbtk::initialize();

    Application::new()
    .window(|ctx| { 
        Window::new()
            .title("Hello, OrbTk!")
            .position((100.0, 100.0))
            .size(200.0, 100.0)
            .child(
                TextBlock::new()
                    .text("Hello, World!")
                    .v_align("center")
                    .h_align("center")
                    .build(ctx)
            )
            .build(ctx)
    })
    .run();
}
