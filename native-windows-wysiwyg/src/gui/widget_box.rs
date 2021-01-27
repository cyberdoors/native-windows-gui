use nwd::NwgPartial;

#[derive(Default)]
#[derive(NwgPartial)]
pub struct WidgetBox {
    #[nwg_control(size: (275, 0))]
    pub(super) container_frame: nwg::Frame,

    #[nwg_layout(spacing: 0, margin: [0,0,0,0])]
    layout: nwg::GridLayout,

    #[nwg_control(parent: container_frame)]
    #[nwg_layout_item(layout: layout, x:0, y:0)]
    pub(super) widgets_tree: nwg::TreeView,
}

impl WidgetBox {

    pub(super) fn load_widgets(&self) {
        let tree = &self.widgets_tree;

        let controls = tree.insert_item("Controls", None, nwg::TreeInsert::Last);
        
        let top_level = tree.insert_item("Top level", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Window", Some(&top_level), nwg::TreeInsert::Last);
        tree.insert_item("Message window", Some(&top_level), nwg::TreeInsert::Last);
        tree.insert_item("Extern canvas (window)", Some(&top_level), nwg::TreeInsert::Last);

        let buttons = tree.insert_item("Buttons", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Button", Some(&buttons), nwg::TreeInsert::Last);
        tree.insert_item("Check box", Some(&buttons), nwg::TreeInsert::Last);
        tree.insert_item("Radio button", Some(&buttons), nwg::TreeInsert::Last);

        let display = tree.insert_item("Display", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Label", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Rich label", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Image frame", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Status bar", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Progress bar", Some(&display), nwg::TreeInsert::Last);

        let inputs = tree.insert_item("Inputs", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Rich text box", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Text box", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Text input", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Number select", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Date picker", Some(&inputs), nwg::TreeInsert::Last);
        
        let collections = tree.insert_item("Collections", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Combobox", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("List box", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("List view", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("Tree view", Some(&collections), nwg::TreeInsert::Last);

        let containers = tree.insert_item("Containers", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Frame", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Tab container", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Tab", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Extern canvas (child)", Some(&containers), nwg::TreeInsert::Last);
    
        let triggers = tree.insert_item("Triggers", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Notice", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Timer", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Tray notification", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Tooltip", Some(&triggers), nwg::TreeInsert::Last);

        let other = tree.insert_item("Other", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Track bar", Some(&other), nwg::TreeInsert::Last);

        let _custom = tree.insert_item("Custom", Some(&controls), nwg::TreeInsert::Last);

        for item in tree.iter() {
            tree.set_expand_state(&item, nwg::ExpandState::Expand);
        }

        tree.set_enabled(false);
    }

}
