mod genome_widget;
mod data;
mod commands;
mod reader;
use data::GUIModel;
use druid::{AppLauncher, PlatformError, Point, Selector, Widget, WidgetExt, WindowDesc, widget::{Button, Flex, Label, Either}};
use genome_widget::{Connection, Genome, GenomeWidget, Node, NodeType};
use im::vector;
use std::rc::Rc;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title("rusty-neat-gui")
        .window_size((1000.0, 800.0));

    let first_node = Rc::from(Node {
        id: 1,
        position: Point::new(0.7, 0.5),
        activation: "Test".to_string(),
        bias: 1.0,
        node_type: NodeType::OUTPUT(1),
    });

    let second_node = Rc::from(Node {
        id: 1,
        position: Point::new(0.2, 0.5),
        activation: "Test".to_string(),
        bias: 1.0,
        node_type: NodeType::INPUT(1),
    });

    let connection = Rc::from(Connection {
        start: Rc::clone(&first_node),
        end: Rc::clone(&second_node),
        innovation: 0,
        enabled: true,
        weight: 10.0
    });

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(GUIModel {
            current_genome: Option::Some(Genome::new(0, vector![Rc::clone(&first_node), Rc::clone(&second_node)], vector![Rc::clone(&connection)]))
        })
}

fn ui_builder() -> impl Widget<GUIModel> {
    Flex::column()
        .with_child(Button::new("Recenter").on_click(|ctx, _data, _env| ctx.submit_command(Selector::new(commands::RECENTER_GENOME_SLECTOR))))
        .with_default_spacer()
        .with_flex_child(Either::new(|data: &GUIModel, env| data.current_genome.is_some(),GenomeWidget::new().lens(GUIModel::current_genome), Label::new("No genome selected")), 1.0)
        .padding(50.0)
}
