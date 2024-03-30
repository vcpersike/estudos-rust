use fltk::{app, prelude::*, window::Window, button::Button};

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 300, 200, "Desligar PC");

    let mut btn_shutdown = Button::new(80, 150, 80, 40, "Desligar");
    let mut btn_cancel = Button::new(180, 150, 80, 40, "Cancelar");

    {
        let mut btn_shutdown_clone = btn_shutdown.clone();
        btn_shutdown.set_callback(move |_| {
            println!("Desligando o PC...");
            // Aqui você colocaria o código para desligar o PC.
            // Em um ambiente real, isso exigiria privilégios administrativos e comandos específicos do sistema operacional.
            btn_shutdown_clone.deactivate(); // Desativa o botão após o clique para evitar cliques múltiplos.
        });
    }

    btn_cancel.set_callback(move |_| {
        println!("Operação cancelada.");
        app.quit();
    });

    win.end();
    win.show();
    app.run().unwrap();
}
