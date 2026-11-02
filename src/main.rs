use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 512,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let paddle_width = 10.;
    let paddle_height = 60.;
    let paddle_wall_clearance = 20.;
    let paddle_default_vertical_position =
        window_conf().window_height as f32 / 2. - paddle_height / 2.;
    let paddle_color = RED;
    let player_paddle = Rect::new(
        paddle_wall_clearance,
        paddle_default_vertical_position,
        paddle_width,
        paddle_height,
    );
    let opponent_paddle = Rect::new(
        window_conf().window_width as f32 - paddle_wall_clearance - paddle_width,
        paddle_default_vertical_position,
        paddle_width,
        paddle_height,
    );
    loop {
        clear_background(WHITE);
        draw_rectangle(
            player_paddle.x,
            player_paddle.y,
            player_paddle.w,
            player_paddle.h,
            paddle_color,
        );
        draw_rectangle(
            opponent_paddle.x,
            opponent_paddle.y,
            opponent_paddle.w,
            opponent_paddle.h,
            paddle_color,
        );
        next_frame().await;
    }
}
