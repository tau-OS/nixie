// Simple macro to make actions easier :)
#[macro_export]
macro_rules! action {
    ($actions_group:expr, $name:expr, $callback:expr) => {{
        let simple_action = gio::SimpleAction::new($name, None);
        simple_action.connect_activate($callback);
        $actions_group.add_action(&simple_action);
    }};
    ($actions_group:expr, $name:expr, $param_type:expr, $callback:expr) => {{
        let simple_action = gio::SimpleAction::new($name, $param_type);
        simple_action.connect_activate($callback);
        $actions_group.add_action(&simple_action);
    }};
}
