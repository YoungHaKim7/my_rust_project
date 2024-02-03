use macroquad::{
    prelude::WHITE,
    shapes::draw_circle,
    text::{draw_text_ex, load_ttf_font, TextParams},
    window::{next_frame, screen_height, screen_width, Conf},
};
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

const NUM_BODIES: usize = 1000;
const ASPECT_RATIO: f32 = 5.0;

const MAX_X: f32 = 5.0E14;
const MIN_X: f32 = -5.0E14;
const MAX_Y: f32 = 5.0E14;
const MIN_Y: f32 = -5.0E14;

const MAX_MASS: f32 = 9.0E29;
const MIN_MASS: f32 = 1.0E15;

const MAX_V: f32 = 9.0E03;
const MIN_V: f32 = 1.0E03;

const GRAVITATION: f32 = 6.67E-11; //만유인력 상수
const D_TIME: f32 = 2.0E07; //수치미분간격(시간: 초)
const D_TIME_HALF: f32 = 1.0E07; //절반(수치미분간격)
const A_RIGHT_YEAR: f32 = 9.46E15; //1광년(m)

#[derive(Clone, Copy, Debug)] //초기화에 Copy 필요
struct Bodies {
    mass: f32,
    x: f32, //k
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    vx_half: f32, //k+1/2
    vy_half: f32,
    x_new: f32, //k+1
    y_new: f32,
    vx_new: f32,
    vy_new: f32,
    ax_new: f32,
    ay_new: f32,
    disp_x: f32,
    disp_y: f32,
}

impl Bodies {
    pub fn new() -> Bodies {
        Bodies {
            mass: 0.,
            x: 0.,
            y: 0.,
            vx: 0.,
            vy: 0.,
            ax: 0.,
            ay: 0.,
            vx_half: 0.,
            vy_half: 0.,
            x_new: 0.,
            y_new: 0.,
            vx_new: 0.,
            vy_new: 0.,
            ax_new: 0.,
            ay_new: 0.,
            disp_x: 0.,
            disp_y: 0.,
        }
    }
}
fn init_bodies(bodies: &mut [Bodies]) -> bool {
    for i in 0..NUM_BODIES {
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        bodies[i].mass = rnd * (MAX_MASS - MIN_MASS) + MIN_MASS;
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        bodies[i].x = rnd * (MAX_X - MIN_X) + MIN_X;
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        bodies[i].y = rnd * (MAX_Y - MIN_Y) + MIN_Y;
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        bodies[i].vx = rnd * (MAX_V - MIN_V) + MIN_V;
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        if rnd < 0.5 {
            bodies[i].vx = -1.0 * bodies[i].vx;
        }
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        bodies[i].vy = rnd * (MAX_V - MIN_V) + MIN_V;
        let rnd: f32 = StdRng::from_entropy().sample(Standard);
        if rnd < 0.5 {
            bodies[i].vy = -1.0 * bodies[i].vy;
        }
    }
    true
}

fn calc_v_half(bodies: &mut [Bodies]) -> bool {
    for i in 0..NUM_BODIES {
        bodies[i].vx_half = bodies[i].vx + bodies[i].ax * D_TIME_HALF;
        bodies[i].vy_half = bodies[i].vy + bodies[i].ay * D_TIME_HALF;
    }
    true
}
fn calc_new_xy(bodies: &mut [Bodies]) -> bool {
    for i in 0..NUM_BODIES {
        bodies[i].x_new = bodies[i].x + bodies[i].vx_half * D_TIME;
        bodies[i].y_new = bodies[i].y + bodies[i].vy_half * D_TIME;
    }
    true
}
fn calc_new_axay(bodies: &mut [Bodies]) -> bool {
    for i in 0..NUM_BODIES {
        bodies[i].ax_new = 0.;
        bodies[i].ay_new = 0.;
        for j in 0..NUM_BODIES {
            if j == i {
                continue;
            }
            let dx_new: f32 = bodies[j].x_new - bodies[i].x_new;
            let dy_new: f32 = bodies[j].y_new - bodies[i].y_new;
            let r_sq_new: f32 = dx_new.powi(2) + dy_new.powi(2);
            let r_org_new: f32 = r_sq_new.sqrt();
            if r_org_new > A_RIGHT_YEAR {
                continue;
            } //거리 1광년 이상은 가속도에 영향을 주지 않는다고 가정
            let a_org_new: f32 = GRAVITATION * bodies[j].mass / r_sq_new;
            let a_org_x_new: f32 = a_org_new * dx_new / r_org_new;
            let a_org_y_new: f32 = a_org_new * dy_new / r_org_new;
            bodies[i].ax_new = bodies[i].ax_new + a_org_x_new;
            bodies[i].ay_new = bodies[i].ay_new + a_org_y_new;
        }
    }
    true
}

fn calc_new_v(bodies: &mut [Bodies]) -> bool {
    for i in 0..NUM_BODIES {
        bodies[i].vx_new = bodies[i].vx_half + bodies[i].ax_new * D_TIME_HALF;
        bodies[i].vy_new = bodies[i].vy_half + bodies[i].ay_new * D_TIME_HALF;
    }
    true
}
fn calc_disp_xy(bodies: &mut [Bodies]) -> bool {
    //우주좌표를 화면좌표로 변환할 계수 설정
    let disp_x_convfactor: f32 = screen_width() / 2.0 / MAX_X / ASPECT_RATIO;
    let disp_y_convfactor: f32 = screen_height() / 2.0 / MAX_Y / ASPECT_RATIO;
    let half_disp_x: f32 = screen_width() / 2.0;
    let half_disp_y: f32 = screen_height() / 2.0;
    //화면좌표 구하기
    for i in 0..NUM_BODIES {
        bodies[i].disp_x = bodies[i].x_new * disp_x_convfactor + half_disp_x;
        bodies[i].disp_y = bodies[i].y_new * disp_y_convfactor + half_disp_y;
    }
    //new(k+1)값을 k값으로 저장
    for i in 0..NUM_BODIES {
        bodies[i].x = bodies[i].x_new;
        bodies[i].y = bodies[i].y_new;
        bodies[i].vx = bodies[i].vx_new;
        bodies[i].vy = bodies[i].vy_new;
        bodies[i].ax = bodies[i].ax_new;
        bodies[i].ay = bodies[i].ay_new;
    }
    true
}

fn window_conf() -> Conf {
    Conf {
        window_title: "만유인력에 의한 항성 운동(LeapFrog 방식 계산결과)Star motion by universal gravitation (the result of the LeapFrog method calculation)".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //한글폰트 설정
    // let font = load_ttf_font("../../font/D2Coding-Ver1.3.2-20180524.ttf")
    //     .await
    //     .unwrap();

    //Bodies 초기화
    let mut bodies = [Bodies::new(); NUM_BODIES];
    init_bodies(&mut bodies);
    let mut elapsed_time: f32 = 0.;
    loop {
        //Kick-Drift-Kick 계산
        calc_v_half(&mut bodies);
        calc_new_xy(&mut bodies);
        calc_new_axay(&mut bodies);
        calc_new_v(&mut bodies);
        calc_disp_xy(&mut bodies); //화면좌표 구한 뒤 k+1값을 k값으로 저장
        for i in 0..NUM_BODIES {
            draw_circle(bodies[i].disp_x, bodies[i].disp_y, 1.0, WHITE);
        }
        elapsed_time = elapsed_time + D_TIME;
        let elapsed_year: f32 = elapsed_time / 3.154E7;
        let s = format!("경과시간elapsed_year:      {:.2E}년", elapsed_year);
        draw_text_ex(
            &s,
            400.0,
            70.0,
            TextParams {
                font_size: 20,
                // font,
                rotation: 0.02, // 라디안
                ..Default::default()
            },
        );
        //운동 에너지 합
        let mut kinetic_energy: f64 = 0.;
        for i in 0..NUM_BODIES {
            let ke: f64 = 0.5
                * bodies[i].mass as f64
                * ((bodies[i].vx).powi(2) + (bodies[i].vy).powi(2)) as f64;
            kinetic_energy = kinetic_energy + ke;
        }
        let s = format!(
            "운동 에너지 합sum of kinetic energy::      {:.2E} J",
            kinetic_energy
        );
        draw_text_ex(
            &s,
            400.0,
            100.0,
            TextParams {
                font_size: 20,
                // font: Option<&'a Font>,
                // font,
                rotation: 0.02, // 라디안
                ..Default::default()
            },
        );
        //위치 에너지 합
        let mut potential_energy: f64 = 0.;
        for i in 0..NUM_BODIES {
            for j in 0..NUM_BODIES {
                if j == i {
                    continue;
                }
                let r2: f64 = (bodies[j].x - bodies[i].x).powi(2) as f64
                    + (bodies[j].y - bodies[i].y).powi(2) as f64;
                let pe: f64 =
                    -1.0 * GRAVITATION as f64 * bodies[i].mass as f64 * bodies[j].mass as f64
                        / r2.sqrt() as f64;
                potential_energy = potential_energy + pe;
            }
        }

        let s = format!(
            "위치 에너지 합sum of potential energy:      {:.2E} J",
            potential_energy
        );
        draw_text_ex(
            &s,
            400.0,
            130.0,
            TextParams {
                font_size: 20,
                // font: Option<&'a font>,
                // font,
                rotation: 0.02, // 라디안
                ..Default::default()
            },
        );
        next_frame().await
    }
}
