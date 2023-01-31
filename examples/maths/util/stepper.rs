use bevy_egui::egui::Ui;

#[derive(Copy, Clone, Debug)]
pub struct Stepper {
    current_step: i32,
    target_step: i32,
}

impl Stepper {
    pub fn new(step: i32) -> Stepper {
        Stepper {
            current_step: 0,
            target_step: step,
        }
    }

    pub fn ui(ui: &mut Ui, step: &mut i32, last_step: i32) {
        ui.label("Step");
        ui.horizontal(|ui| {
            if ui.button("0").clicked() {
                *step = 0;
            }
            if ui.button("-").clicked() {
                *step -= 1;
            }
            if ui.button("+").clicked() {
                *step += 1;
            }
            *step = (*step).clamp(0, last_step);
            ui.label(format!("{}", *step));
        });
    }

    pub fn show(&mut self) -> bool {
        self.current_step += 1;
        self.target_step == self.current_step
    }

    pub fn last_step(&self) -> i32 {
        self.current_step
    }
}
