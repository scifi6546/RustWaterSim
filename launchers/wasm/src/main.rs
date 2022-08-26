use bevy::prelude::*;
use game_plugin::GamePlugin;
use stylist::{css, global_style, yew::styled_component};
use yew::prelude::*;
#[styled_component(Root)]
fn view() -> Html {
    web_sys::window()
        .map(|w| w.document())
        .flatten()
        .expect("failed to get DOM")
        .set_title("RUST");
    global_style! {
        r#"
        html{
            min-height:100%;
            position:relative;

        }
        body{
            height:100%;
            padding:0;
            margin:0;
        }
        "#


    }
    .expect("failed to set global style");
    let css = css!(
        r#"
    position:absolute;
    overflow:hidden;
    width:100%;
    height:100%;
        "#
    );
    html! {<div class={css}>
        <canvas id="bevy"></canvas>

    </div>}
}
fn main() {
    yew::start_app::<Root>();
    let mut app = App::new();
    app
      //  .add_plugin(GamePlugin)
        .insert_resource(WindowDescriptor {
            title: "RUST".to_string(), // ToDo
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
          //  height:100.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
             .add_plugin(GamePlugin).insert_resource(bevy::winit::WinitSettings::game())
        //       .insert_resource(Msaa { samples: 4 })
        ;

    app.run();
}
