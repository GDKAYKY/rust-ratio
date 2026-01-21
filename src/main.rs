use eframe::egui;
use std::f32::consts::PI;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("Infinite Fibonacci Zoom"),
        ..Default::default()
    };
    eframe::run_native(
        "Infinite Fibonacci Zoom",
        options,
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}

struct App {
    time: f32,
    phi: f32,
    cycle: i32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            time: 0.0,
            phi: (1.0 + 5.0f32.sqrt()) / 2.0,
            cycle: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.time += 0.0075;

        // Reset a cada ~20 segundos para manter os números gerenciáveis
        if self.time > 20.0 {
            self.time = 0.0;
            self.cycle += 1;
        }

        ctx.request_repaint();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                let painter = ui.painter();
                let rect = ui.available_rect_before_wrap();
                let center = rect.center();

                let zoom = self.phi.powf(self.time);
                let current_scale = 160.0 / zoom;

                let mut fib = vec![1.0, 1.0];
                for _ in 0..50 {
                    let n = fib.len();
                    let next = fib[n - 1] + fib[n - 2];
                    if next > 1e15 {
                        break;
                    }
                    fib.push(next);
                }

                let eye = egui::pos2(0.723606, 0.276393);
                let mut cur_min = egui::pos2(0.0, 0.0);
                let mut cur_max = egui::pos2(1.0, 1.0);

                for (i, &f) in fib.iter().enumerate() {
                    let s = f as f32;
                    let (sq_pos, arc_center, start_angle) = if i == 0 {
                        (egui::pos2(0.0, 0.0), egui::pos2(0.0, 1.0), 0.0)
                    } else {
                        match i % 4 {
                            1 => {
                                let p = egui::pos2(cur_min.x, cur_min.y - s);
                                (p, egui::pos2(p.x, p.y + s), 1.5 * PI)
                            }
                            2 => {
                                let p = egui::pos2(cur_min.x - s, cur_min.y);
                                (p, egui::pos2(p.x + s, p.y + s), PI)
                            }
                            3 => {
                                let p = egui::pos2(cur_min.x, cur_max.y);
                                (p, egui::pos2(p.x + s, p.y), 0.5 * PI)
                            }
                            0 => {
                                let p = egui::pos2(cur_max.x, cur_min.y);
                                (p, egui::pos2(p.x, p.y), 0.0)
                            }
                            _ => unreachable!(),
                        }
                    };

                    let sq_rect = egui::Rect::from_min_size(sq_pos, egui::vec2(s, s));
                    cur_min.x = cur_min.x.min(sq_rect.min.x);
                    cur_min.y = cur_min.y.min(sq_rect.min.y);
                    cur_max.x = cur_max.x.max(sq_rect.max.x);
                    cur_max.y = cur_max.y.max(sq_rect.max.y);

                    let transform = |p: egui::Pos2| center + (p - eye) * current_scale;
                    let screen_rect =
                        egui::Rect::from_min_max(transform(sq_rect.min), transform(sq_rect.max));
                    let s_px = screen_rect.width();

                    if s_px < 0.1 {
                        continue;
                    }
                    if s_px > rect.width() * 20.0 {
                        break;
                    }

                    let alpha = if s_px < 20.0 {
                        ((s_px / 20.0) * 255.0) as u8
                    } else if s_px > rect.width() * 2.0 {
                        (255.0
                            - ((s_px - rect.width() * 2.0) / (rect.width() * 10.0) * 255.0)
                                .min(255.0)) as u8
                    } else {
                        255
                    };

                    painter.rect_stroke(
                        screen_rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::from_white_alpha(alpha / 4)),
                    );

                    if s_px > 40.0 && alpha > 50 {
                        painter.text(
                            screen_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", f as i64),
                            egui::FontId::proportional(s_px.min(40.0).max(12.0)),
                            egui::Color32::from_white_alpha(alpha),
                        );
                    }

                    let steps = 80;
                    let mut points = Vec::new();
                    for step in 0..=steps {
                        let t = step as f32 / steps as f32;
                        let angle = start_angle + t * (PI / 2.0);
                        let p_model = arc_center + egui::vec2(s * angle.cos(), s * angle.sin());
                        points.push(transform(p_model));
                    }

                    painter.add(egui::Shape::line(
                        points.clone(),
                        egui::Stroke::new(
                            6.0,
                            egui::Color32::from_rgba_premultiplied(255, 140, 0, alpha / 6),
                        ),
                    ));

                    painter.add(egui::Shape::line(
                        points,
                        egui::Stroke::new(
                            3.0,
                            egui::Color32::from_rgba_premultiplied(255, 160, 20, alpha),
                        ),
                    ));
                }
            });
    }
}
