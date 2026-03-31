/* 아무래도 elu는 rust단계에선 위험할듯 싶음 일단 relu로 해보고 그다음 elu해보는게 좋을듯 (exp <-- 이싯키때매)
pub fn elu(x: f32) -> f32{
    if x > 0.0 {
        x
    } else {
        x.exp() - 1.0
    }
}
*/

pub fn relu(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

fn relu_derivative(x: f32) -> f32 { //이게 미분 적용된거.
    if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn mse(x: f32, target: f32) -> f32 {
    (x - target).powi(2)
}

pub fn mse_derivative(x: f32, target: f32) -> f32 {
    2.0 * (x - target)
}