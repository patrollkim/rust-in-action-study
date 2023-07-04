use num::complex::Complex; // Complex 숫자 타입을 num::complex(복소수) 하위 모듈에서 가져온다.

// 출력공간(행과 열의 그리드)과 망델브로 집합[(0,0)에 가까운 연속 영역]을 둘러싼 범위를 변환한다.
fn calculate_mandelbrot(
    max_iters: usize, // 값이 최대 반복 횟수에 도달하기 전에 빠져나오지 않은 경우 망델브로 집합 내에 있는 것으로 간주한다.
    x_min: f64, // --
    x_max: f64, // --
    y_min: f64, // --
    y_max: f64, // -- 집합의 멤버를 찾기 위해 검색할 공간을 지정하는 매개변수
    width: usize,  // --
    height: usize, // -- 출력 크기를 픽셀로 나타내는 매개변수
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width); // 각 행의 데이터를 저장할 컨테이너를 생성한다.
    for img_y in 0..height { // 한 행씩 반복하여 한 줄씩 출력하도록 한다.
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = (img_x as f64 / width as f64);
            let y_percent = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * x_percent; // --
            let cy = y_min + (y_max - y_min) * y_percent; // -- 출력에서 다루는 공간의 비율을 계산하여 검색 공간 내의 점으로 변환한다.
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows

}

// 모든 픽셀에서 호출되는 함수(예: stdout으로 출력되는 모든 행과 열).
fn mandelbrot_at_point(
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 }; // 0.0에서 실수부(re)와 허수부(im)를 사용하여 원점에서 복소수를 초기화 한다.
    let c = Complex::new(cx, cy); // 함수 인자로 제공된 좌표에서 복소수를 초기화한다.

    for i in 0..= max_iters {
        if z.norm() > 2.0 { // 탈출 조건을 점검하고 복소수의 절댓값, 원점(0,0)과의 거리를 계산한다.
            return i;
        }
        z = z * z + c; // z를 반복적으로 변경해 c가 망델브로 집합 내에 있는지 확인한다.
    }
    max_iters // ㅑi가 저 이상 영역 내에 없으므로 max_iters로 대신한다.
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '∙',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'X',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000,
                                            -2.0, 
                                            1.0, 
                                            -1.0, 
                                            1.0, 
                                            100, 
                                            24);
    
    render_mandelbrot(mandelbrot);
}
