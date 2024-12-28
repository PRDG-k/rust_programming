use num::complex::Complex;

// #[proc-macro]
fn calculate_mandelbrot(
    
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,

) -> Vec<Vec<usize>> {

    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for img_x in 0..width {

            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            // 어느 지점(cx, cy)에서 발산하는지 확인하고 계산을 멈춤
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at)
        }

        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(

    cx: f64,
    cy: f64,
    max_iters: usize,

) -> usize {
    
    let mut z = Complex { re:0.0, im: 0.0 };
    let c = Complex::new(cx,cy);

    for i in 0..=max_iters{

        // 경계값!
        // z의 절댓값이 2보다 크면 발산한다고 말 할 수 있다.
        // 이떄의 c는 망델브로 집합에 속하지 않는다.
        if z.norm() > 2.0 {
            return i;
        }

        // 망델브로 점화식. 집합을 컨트롤하는 것은 복소수 c 이다.
        // 변수 c에 따라서 집합의 발산속도가 달라진다.
        // 이때 발산속도가 늦어질 수록 망델브로 집합에 가깝다.

        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '`',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}

struct Config {

    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
    
}

fn main() {
    let config = Config{

        max_iters: 1000,
        x_min: -2.0,
        x_max: 1.0,
        y_min: -1.0,
        y_max: 1.0,
        width: 100,
        height: 24,

    };

    // 패턴 매칭
    let Config {max_iters, x_min, x_max, y_min, y_max, width, height} = config;
    let mandelbrot = calculate_mandelbrot(

        max_iters,
        x_min,
        x_max,
        y_min,
        y_max,
        width,
        height

    );

    render_mandelbrot(mandelbrot)
}
