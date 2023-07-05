use nannou::prelude::*;
use nannou_egui::{egui::{self, FontDefinitions, FontFamily, Color32, Label}, Egui, FrameCtx};
const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 800.0;


struct ParticleOne {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
    r: f32,
}
impl ParticleOne {
    fn new(x: f32, y: f32, r: f32) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let position = pt2(x, y);
        let life_span = 255.0;
        ParticleOne {
            acceleration,
            velocity,
            position,
            life_span,
            r,
        }
    }
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 2.0;
    }
    fn display(&self, draw: &Draw) {
        draw.rect().xy(self.position).w_h(self.r, self.r).rgba(
            0.0,
            0.0,
            0.0,
            self.life_span / 255.0,
        );
    }
    fn _is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}
struct OneParticleSystem {
    particles: Vec<ParticleOne>,
    intact: bool,
}
impl OneParticleSystem {
    fn new(x: f32, y: f32, r: f32) -> Self {
        let particles = Vec::new();
        let rows = 50;
        let cols = 50;

        let mut ps = OneParticleSystem {
            particles,
            intact: true,
        };
        for i in 0..(rows * cols) {
            ps.add_particle(x + (i % cols) as f32 * r, y - (i / rows) as f32 * r, r);
        }
        ps
    }

    fn add_particle(&mut self, x: f32, y: f32, r: f32) {
        self.particles.push(ParticleOne::new(x, y, r));
    }

    fn shatter(&mut self) {
        self.intact = false;
    }
    fn update(&mut self) {
        if !self.intact {
            for i in (0..self.particles.len()).rev() {
                self.particles[i].update();
            }
        }
    }
    fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}
#[derive(Clone)]
struct ParticleThree {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
    r: f32,
    idx: u64,
    resolution:u32,
    color:Hsv
}
impl ParticleThree {
    fn new(l: Point2, idx: u64,coffee:&Coffee) -> Self {
        let acceleration = vec2(0.0, 0.0);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() * 2.0 - 1.0);
        let position = l;
        let life_span = 255.0;
        ParticleThree {
            acceleration,
            velocity,
            position,
            life_span,
            resolution:coffee.resolution,
            r: coffee.radius,
            color:coffee.color,
            // r: 20.0,
            idx,
        }
    }

    fn intersects(&mut self, particles: &Vec<ParticleThree>) {
        for i in 0..particles.len() {
            if particles[i].idx != self.idx {
                let mut dir = self.position - particles[i].position;
                if dir.length() < self.r {
                    dir = dir.normalize() * 0.5;
                    self.apply_force(dir);
                }
            }
        }
    }

    fn apply_force(&mut self, f: Vec2) {
        self.acceleration += f;
    }

    // Method to update position
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.acceleration *= 0.0;
        self.life_span -= 0.5;
    }

    // Method to display
    fn display(&self, draw: &Draw) {
        let temp = self.color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0);
        draw.ellipse()
            .xy(self.position)
            .resolution(self.resolution.to_f32().unwrap())
            .radius(self.r)
            .hsva(temp, self.color.saturation, self.color.value, self.life_span / 255.0);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}
struct ThreeParticleSystem {
    particles: Vec<ParticleThree>,
    pub origin: Point2,
}

struct ParticleTwo {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
}
impl ThreeParticleSystem {
    fn new(position: Point2) -> Self {
        let origin = position;
        let particles = Vec::new();
        ThreeParticleSystem { origin, particles }
    }

    fn add_particle(&mut self, idx: u64,coffee:&Coffee) {
        self.particles.push(ParticleThree::new(self.origin, idx,coffee));
    }

    fn _apply_force(&mut self, f: Vec2) {
        for i in 0..self.particles.len() {
            self.particles[i].apply_force(f);
        }
    }

    fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    fn intersection(&mut self) {
        let particles = self.particles.clone();
        for i in 0..self.particles.len() {
            self.particles[i].intersects(&particles);
        }
    }

    fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}
impl ParticleTwo {
    fn new(l: Point2) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let position = l;
        let life_span = 255.0;
        ParticleTwo {
            acceleration,
            velocity,
            position,
            life_span,
        }
    }

    // Method to update position
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 1.5;
    }
    // Method to display
    fn display(&self, draw: &Draw, coffee:&Coffee) {
        let temp = coffee.color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0);
        draw.ellipse()
            .xy(self.position)
            .resolution(coffee.resolution.to_f32().unwrap())
            .w_h(coffee.radius, coffee.radius)
            .hsva(temp, coffee.color.saturation, coffee.color.value, self.life_span / 255.0);
            // .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            // .stroke_weight(2.0);
    }

    // Is the poarticel still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}
struct ParticleFour {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
    radius:f32,
    color:Hsv,
    resolution:u32
}
impl ParticleFour {
    fn new(l: Point2,coffee:&Coffee) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let position = l;
        let life_span = 255.0;
        let radius = coffee.radius;
        let color = coffee.color;
        let resolution = coffee.resolution;
        ParticleFour {
            acceleration,
            velocity,
            position,
            life_span,
            radius,
            color,
            resolution,
        }
    }

    // Method to update position
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 2.0;
    }

    // Method to display
    fn display(&self, draw: &Draw) {
        let temp = self.color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0);
        draw.ellipse()
            .xy(self.position)
            .resolution(self.resolution.to_f32().unwrap())
            .w_h(self.radius, self.radius)
            .hsva(temp, self.color.saturation, self.color.value, self.life_span / 255.0);
            // .rgba(0.5, 0.5, 0.5, self.life_span / 255.0);
            // .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            // .stroke_weight(2.0);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}
struct FourParticleSystem {
    particles: Vec<ParticleFour>,
    pub origin: Point2,
}
impl FourParticleSystem {
    fn new(position: Point2) -> Self {
        let origin = position;
        let particles = Vec::new();
        FourParticleSystem { origin, particles }
    }

    fn add_particle(&mut self,coffee:&Coffee) {
        self.particles.push(ParticleFour::new(self.origin,coffee));
    }

    fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    fn draws(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}

struct Coffee{
    radius: f32,
    color: Hsv,
    resolution: u32,
}
impl Coffee {
    fn new()->Coffee{
        Coffee { 
            radius: 40.0, 
            color: hsv(10.0, 0.5, 1.0), 
            resolution: 10, 
        }
    }
}
#[derive(PartialEq)]
enum AppState{
    SETTING,
    SHOWONE,
    SHOWTWO,
    SHOWTHREE,
    SHOWFOUR
}
struct Model {
    egui: Egui,
    state:AppState,
    coffee:Coffee,
    particlestwos: Vec<ParticleTwo>,
    ps: OneParticleSystem,
    psthree: ThreeParticleSystem,
    psfour: FourParticleSystem,
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let window_id = app
        .new_window()
        .title("COFFEE")
        .size(WIDTH as u32, HEIGHT as u32)
        .raw_event(raw_window_event) // This is where we forward all raw events for egui to process them
        .view(view)
        .mouse_pressed(mouse_pressed) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();
    let win = app.window_rect();
    let (_w, h) = app.window_rect().w_h();
    let window = app.window(window_id).unwrap();
    
    Model {
        egui: Egui::from_window(&window),
        state: AppState::SETTING,
        coffee:Coffee::new(),
        particlestwos:Vec::new(),
        ps: OneParticleSystem::new(win.left() + 400.0, win.top() - 100.0, 5.0),
        psthree:ThreeParticleSystem::new(pt2(0.0, 0.0)),
        psfour: FourParticleSystem::new(pt2(0.0, (h as f32 / 2.0) - 50.0)),
        
    }
}
fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let coffee = &mut model.coffee;
    let appstate = &mut model.state;
    let particlestwos = &mut model.particlestwos;
    // let psthree = &mut model.psthree;
    match *appstate {
        AppState::SETTING=>{},
        AppState::SHOWONE=>{
            model.ps.update();
        },
        AppState::SHOWTWO=>{
            particlestwos
                .push(ParticleTwo::new(pt2(0.0, _app.window_rect().top() - 50.0)));
            for i in (0..particlestwos.len()).rev() {
                particlestwos[i].update();
                if particlestwos[i].is_dead() {
                    particlestwos.remove(i);
                }
            }
        },
        AppState::SHOWTHREE=>{
            let win = _app.window_rect();
            model.psthree.origin = pt2(
                random_range(win.left(), win.right()),
                random_range(win.bottom(), win.top()),
            );
            //let gravity = pt2(0.0, 0.1);
            //m.ps.apply_force(gravity);

            model.psthree.add_particle(_app.elapsed_frames(),&coffee);
            model.psthree.update();
            model.psthree.intersection();
            
        },
        AppState::SHOWFOUR=>{
            model.psfour.origin = pt2(_app.mouse.x, _app.mouse.y);
            model.psfour.add_particle(&coffee);
            model.psfour.update();
        },

    }
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("my_font".to_owned(),
    std::borrow::Cow::Borrowed(include_bytes!("../koryungddal.ttf"))); // .ttf and .otf supported

    fonts.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap()
    .insert(0, "my_font".to_owned());
    fonts.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
    egui::Window::new("커피를 만들어봅시다")
        .default_size(egui::vec2(0.0, 200.0))
        .show(&ctx, |ui| {
            if ui.button("커미 꾸미기").clicked(){
                *appstate=AppState::SETTING;
            };
            ui.add(Label::new("모양").heading().text_color(crate::Color32::LIGHT_RED));
            ui.add(egui::Slider::new(&mut coffee.resolution, 1..=40));
            ui.separator();
            ui.add(Label::new("크기").heading().text_color(crate::Color32::LIGHT_RED));
            ui.add(egui::Slider::new(&mut coffee.radius, 10.0..=100.0).text("Radius"));
            // ui.label(RichText::new("asd").color(Color32::from_rgb(110, 255, 110)));
            ui.separator();
            ui.add(Label::new("색상").heading().text_color(crate::Color32::LIGHT_RED));
            edit_hsv(ui, &mut coffee.color);
            ui.separator();
            ui.add(Label::new("보기").heading().text_color(crate::Color32::LIGHT_RED));
            ui.horizontal_wrapped(|ui|{
                if ui.button("티백").clicked(){
                    let win = _app.window_rect();
                    model.ps=OneParticleSystem::new(win.left() + 400.0, win.top() - 100.0, 5.0);
                    *appstate=AppState::SHOWONE;
                };
                if ui.button("뿌리기").clicked(){
                    *appstate=AppState::SHOWTWO;
                };
                if ui.button("퍼져라").clicked(){
                    model.psthree=ThreeParticleSystem::new(pt2(0.0, 0.0));
                    *appstate=AppState::SHOWTHREE;
                };
                if ui.button("따라가기").clicked(){
                    let (_w, h) = _app.window_rect().w_h();
                    // model.psfour =FourParticleSystem::new(pt2(0.0, (h as f32 / 2.0) - 50.0));
                    *appstate=AppState::SHOWFOUR;
                };
            });
        });
}
fn main() {
    nannou::app(model).update(update).run();
}


fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    match model.state {
        AppState::SETTING=>{
            let draw = app.draw();
            frame.clear(WHITE);
            draw.ellipse()
                .resolution(model.coffee.resolution as f32)
                .x_y(100.0, 100.0)
                .radius(model.coffee.radius)
                .color(model.coffee.color);
            draw.to_frame(app, &frame).unwrap();
            // Do this as the last operation on your frame.
            model.egui.draw_to_frame(&frame).unwrap();
        },
        AppState::SHOWONE=>{
            let draw = app.draw();
            frame.clear(WHITE);
            model.ps.draw(&draw);
            draw.to_frame(app, &frame).unwrap();
            // draw.to_frame(app, &frame).unwrap();
            
            model.egui.draw_to_frame(&frame).unwrap();
            
        },
        AppState::SHOWTWO=>{
            let draw = app.draw();
            frame.clear(WHITE);
            draw.to_frame(app, &frame).unwrap();
            model.egui.draw_to_frame(&frame).unwrap();
            for p in model.particlestwos.iter() {
                p.display(&draw,&model.coffee);
            }
            draw.to_frame(app, &frame).unwrap();
        },
        AppState::SHOWTHREE=>{
            let draw = app.draw();
            frame.clear(WHITE);
            // 
            model.psthree.draw(&draw);
            draw.to_frame(app, &frame).unwrap();
            model.egui.draw_to_frame(&frame).unwrap();
        },
        AppState::SHOWFOUR=>{
            let draw = app.draw();
            frame.clear(WHITE);
            model.psfour.draws(&draw);
            draw.to_frame(app, &frame).unwrap();
            model.egui.draw_to_frame(&frame).unwrap();
        }
    }
}

fn edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv = egui::color::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
        color.saturation,
        color.value,
        1.0,
    );

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = nannou::color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {
    if _button==MouseButton::Right{
        m.ps.shatter();
    }
    
}
