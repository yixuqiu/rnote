pub mod brushpage;
pub mod eraserpage;
pub mod markerpage;
pub mod selectorpage;
pub mod shaperpage;

mod imp {
    use super::{
        brushpage::BrushPage, eraserpage::EraserPage, markerpage::MarkerPage,
        selectorpage::SelectorPage, shaperpage::ShaperPage,
    };
    use crate::ui::{colorpicker::ColorPicker, templatechooser::TemplateChooser};
    use gtk4::{
        glib, prelude::*, subclass::prelude::*, CompositeTemplate, Stack, StackPage, Widget,
    };

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/penssidebar.ui")]
    pub struct PensSideBar {
        #[template_child]
        pub sidebar_stack: TemplateChild<Stack>,
        #[template_child]
        pub marker_stackpage: TemplateChild<StackPage>,
        #[template_child]
        pub marker_page: TemplateChild<MarkerPage>,
        #[template_child]
        pub brush_stackpage: TemplateChild<StackPage>,
        #[template_child]
        pub brush_page: TemplateChild<BrushPage>,
        #[template_child]
        pub shaper_stackpage: TemplateChild<StackPage>,
        #[template_child]
        pub shaper_page: TemplateChild<ShaperPage>,
        #[template_child]
        pub eraser_stackpage: TemplateChild<StackPage>,
        #[template_child]
        pub eraser_page: TemplateChild<EraserPage>,
        #[template_child]
        pub selector_stackpage: TemplateChild<StackPage>,
        #[template_child]
        pub selector_page: TemplateChild<SelectorPage>,
    }

    impl Default for PensSideBar {
        fn default() -> Self {
            Self {
                sidebar_stack: TemplateChild::<Stack>::default(),
                marker_stackpage: TemplateChild::<StackPage>::default(),
                marker_page: TemplateChild::<MarkerPage>::default(),
                brush_stackpage: TemplateChild::<StackPage>::default(),
                brush_page: TemplateChild::<BrushPage>::default(),
                shaper_stackpage: TemplateChild::<StackPage>::default(),
                shaper_page: TemplateChild::<ShaperPage>::default(),
                eraser_stackpage: TemplateChild::<StackPage>::default(),
                eraser_page: TemplateChild::<EraserPage>::default(),
                selector_stackpage: TemplateChild::<StackPage>::default(),
                selector_page: TemplateChild::<SelectorPage>::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PensSideBar {
        const NAME: &'static str = "PensSideBar";
        type Type = super::PensSideBar;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

            ColorPicker::static_type();
            TemplateChooser::static_type();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PensSideBar {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for PensSideBar {}
}

use crate::ui::appwindow::RnoteAppWindow;

use brushpage::BrushPage;
use eraserpage::EraserPage;
use markerpage::MarkerPage;
use selectorpage::SelectorPage;
use shaperpage::ShaperPage;

use gtk4::{glib, glib::clone, prelude::*, subclass::prelude::*, Stack, StackPage, Widget};

glib::wrapper! {
    pub struct PensSideBar(ObjectSubclass<imp::PensSideBar>)
        @extends Widget;
}

impl Default for PensSideBar {
    fn default() -> Self {
        Self::new()
    }
}

impl PensSideBar {
    pub fn new() -> Self {
        let penssidebar: PensSideBar =
            glib::Object::new(&[]).expect("Failed to create PensSideBar");
        penssidebar
    }

    pub fn sidebar_stack(&self) -> Stack {
        imp::PensSideBar::from_instance(self).sidebar_stack.get()
    }

    pub fn marker_stackpage(&self) -> StackPage {
        imp::PensSideBar::from_instance(self).marker_stackpage.get()
    }

    pub fn marker_page(&self) -> MarkerPage {
        imp::PensSideBar::from_instance(self).marker_page.get()
    }

    pub fn brush_stackpage(&self) -> StackPage {
        imp::PensSideBar::from_instance(self).brush_stackpage.get()
    }

    pub fn brush_page(&self) -> BrushPage {
        imp::PensSideBar::from_instance(self).brush_page.get()
    }

    pub fn shaper_page(&self) -> ShaperPage {
        imp::PensSideBar::from_instance(self).shaper_page.get()
    }

    pub fn eraser_stackpage(&self) -> StackPage {
        imp::PensSideBar::from_instance(self).eraser_stackpage.get()
    }

    pub fn eraser_page(&self) -> EraserPage {
        imp::PensSideBar::from_instance(self).eraser_page.get()
    }

    pub fn selector_stackpage(&self) -> StackPage {
        imp::PensSideBar::from_instance(self)
            .selector_stackpage
            .get()
    }

    pub fn selector_page(&self) -> SelectorPage {
        imp::PensSideBar::from_instance(self).selector_page.get()
    }

    pub fn init(&self, appwindow: &RnoteAppWindow) {
        let priv_ = imp::PensSideBar::from_instance(self);

        priv_.sidebar_stack.get().connect_visible_child_name_notify(
            clone!(@weak appwindow => move |sidebar_stack| {
                if let Some(child_name) = sidebar_stack.visible_child_name() {
                    match child_name.to_value().get::<String>().unwrap().as_str() {
                        "marker_page" => {
                            appwindow.application().unwrap().activate_action("current-pen", Some(&"marker".to_variant()));
                        },
                        "brush_page" => {
                            appwindow.application().unwrap().activate_action("current-pen", Some(&"brush".to_variant()));
                        },
                        "shaper_page" => {
                            appwindow.application().unwrap().activate_action("current-pen", Some(&"shaper".to_variant()));
                        },
                        "eraser_page" => {
                            appwindow.application().unwrap().activate_action("current-pen", Some(&"eraser".to_variant()));
                        }
                        "selector_page" => {
                            appwindow.application().unwrap().activate_action("current-pen", Some(&"selector".to_variant()));
                        }
                        _ => {}
                    };
                };
            }),
        );

        self.marker_page().init(appwindow);
        self.brush_page().init(appwindow);
        self.shaper_page().init(appwindow);
        self.eraser_page().init(appwindow);
        self.selector_page().init(appwindow);
    }
}