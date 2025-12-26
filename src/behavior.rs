use std::{ collections::HashSet, time::Instant };

use crate::sprite::{
    DesktopGremlin,
    GremlinTask,
    MouseButton,
    MouseKeysState,
    Event as Event,
    get_window_pos,
};

pub trait Behavior {
    /// Called every frame and passes the whole execution ctx
    fn update(&mut self, gremlin: &mut DesktopGremlin, events: &HashSet<Event>);
    // ayy unity
}

#[derive(Debug, Clone)]
pub struct GremlinDrag {
    is_dragging: bool,
    key_state: MouseKeysState,
    move_torwards_cursor: bool,
    last_moved_at: Instant,
    should_check_drag: bool,
    drag_start_x: i32,
    drag_start_y: i32,
}

impl GremlinDrag {
    pub fn setup() -> Self {
        Self {
            is_dragging: Default::default(),
            key_state: Default::default(),
            move_torwards_cursor: Default::default(),
            last_moved_at: Instant::now(),
            should_check_drag: Default::default(),
            drag_start_x: Default::default(),
            drag_start_y: Default::default(),
        }
    }
}

impl Behavior for GremlinDrag {
    fn update(&mut self, context: &mut DesktopGremlin, events: &HashSet<Event>) {
        for event in events {
            match event {
                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            self.key_state.lmb = true;
                        }
                        _ => (),
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    if self.key_state.lmb && !self.is_dragging {
                        self.is_dragging = true;
                        let _ = context.task_channel.0.send(
                            GremlinTask::PlayInterrupt("GRAB".to_string())
                        );
                        context.task_queue.clear();
                        (self.drag_start_x, self.drag_start_y) = (*x, *y);
                    }
                    if self.is_dragging && self.should_check_drag {
                        let (gremlin_x, gremlin_y) = get_window_pos(&context.canvas);
                        context.canvas
                            .window_mut()
                            .set_position(
                                sdl3::video::WindowPos::Positioned(
                                    gremlin_x.saturating_add((x - self.drag_start_x) as i32)
                                ),
                                sdl3::video::WindowPos::Positioned(
                                    gremlin_y.saturating_add((y - self.drag_start_y) as i32)
                                )
                            );
                    }
                    // only move every odd frame because moving the window will trigger another mousemove event
                    self.should_check_drag = !self.should_check_drag;
                }

                Event::MouseButtonUp { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            if !self.is_dragging && self.key_state.lmb {
                                let _ = context.task_channel.0.send(
                                    GremlinTask::PlayInterrupt("CLICK".to_string())
                                );
                                self.move_torwards_cursor = !self.move_torwards_cursor;
                                self.last_moved_at = Instant::now();
                            }
                            if self.is_dragging && self.key_state.lmb {
                                let _ = context.task_channel.0.send(
                                    GremlinTask::PlayInterrupt("PAT".to_string())
                                );
                            }
                            let _ = context.task_channel.0.send(
                                GremlinTask::Play("IDLE".to_string())
                            );
                            self.is_dragging = false;
                            self.key_state.lmb = false;
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct GremlinMove {
    velocity: f32,
}

impl Default for GremlinMove {
    fn default() -> Self {
        Self { velocity: 250.0 }
    }
}
impl Behavior for GremlinMove {
    fn update(&mut self, _: &mut DesktopGremlin, _: &HashSet<Event>) {
        todo!()
    }
}
