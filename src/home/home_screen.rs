use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::home::main_mobile_ui::MainMobileUI;
    import crate::home::rooms_sidebar::RoomsSideBar;
    import crate::home::spaces_dock::SpacesDock;
    import crate::shared::styles::*;
    import crate::shared::adaptive_view::AdaptiveView;
    import crate::shared::search_bar::SearchBar;
    import crate::home::main_desktop_ui::MainDesktopUI;

    NavigationWrapper = {{NavigationWrapper}} {
        view_stack = <StackNavigation> {}
    }

    HomeScreen = <AdaptiveView> {
        Desktop = {
            show_bg: true
            draw_bg: {
                color: (COLOR_PRIMARY)
            }
            width: Fill, height: Fill
            padding: 0, margin: 0, align: {x: 0.0, y: 0.0}
            flow: Right

            spaces = <SpacesDock> {}

            <View> {
                flow: Down
                width: Fill, height: Fill
                <SearchBar> {}
                <MainDesktopUI> {}
            }
        }

        Mobile = {
            show_bg: true
            draw_bg: {
                color: (COLOR_PRIMARY)
            }
            width: Fill, height: Fill
            flow: Down

            <NavigationWrapper> {
                view_stack = <StackNavigation> {
                    root_view = {
                        padding: {top: 40.}
                        flow: Down
                        width: Fill, height: Fill
                        sidebar = <RoomsSideBar> {}
                        spaces = <SpacesDock> {}
                    }

                    main_content_view = <StackNavigationView> {
                        width: Fill, height: Fill
                        header = {
                            content = {
                                button_container = {
                                    padding: {left: 14}
                                }
                                title_container = {
                                    title = {
                                        draw_text: {
                                            color: (ROOM_NAME_TEXT_COLOR)
                                        }
                                    }
                                }
                            }
                        }
                        body = {
                            main_content = <MainMobileUI> {}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct NavigationWrapper {
    #[deref]
    view: View,
}

impl Widget for NavigationWrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for NavigationWrapper {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.stack_navigation(id!(view_stack))
            .handle_stack_view_actions(cx, actions);
    }
}
