use bevy_egui::egui::Ui;

#[derive(Default, Copy, Clone, Debug)]
pub struct StepperConfig {
    pub step: i32,
    pub show: bool,
}

impl StepperConfig {
    pub fn begin(&self) -> Stepper {
        if self.show {
            Stepper::new(self.step)
        } else {
            Stepper::new_disabled()
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, last_step: i32) {
        ui.label("Steps");
        ui.checkbox(&mut self.show, "Show");
        if self.show {
            ui.horizontal(|ui| {
                if ui.button("<<").clicked() {
                    self.step = 0;
                }
                if ui.button("-").clicked() {
                    self.step -= 1;
                }
                if ui.button("+").clicked() {
                    self.step += 1;
                }
                if ui.button(">>").clicked() {
                    self.step = last_step;
                }
                self.step = self.step.clamp(0, last_step);
                ui.label(format!("{}", self.step));
            });
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Stepper {
    Enabled { current_step: i32, target_step: i32 },
    Disabled,
}

impl Stepper {
    pub fn new(step: i32) -> Stepper {
        Stepper::Enabled {
            current_step: 0,
            target_step: step,
        }
    }

    pub fn new_disabled() -> Stepper {
        Stepper::Disabled
    }

    pub fn show_step(&mut self) -> bool {
        match self {
            Stepper::Enabled {
                current_step,
                target_step,
            } => {
                *current_step += 1;
                *target_step == *current_step
            }
            Stepper::Disabled => false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        matches!(*self, Stepper::Enabled { .. })
    }

    pub fn last_step(&self) -> i32 {
        match self {
            Stepper::Enabled { current_step, .. } => *current_step,
            Stepper::Disabled => 0,
        }
    }
}
