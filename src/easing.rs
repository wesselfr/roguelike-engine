#![allow(dead_code)]

pub fn ease_in_sine(val: f32) -> f32 {
    1.0 - ((val * std::f32::consts::PI) / 2.0).cos()
}

pub fn ease_out_sin(val: f32) -> f32 {
    ((val * std::f32::consts::PI) / 2.0).sin()
}

pub fn ease_in_out_sin(val: f32) -> f32 {
    -((std::f32::consts::PI * val) - 1.0).cos() / 2.0
}

pub fn ease_in_quad(val: f32) -> f32 {
    val * val
}

pub fn ease_out_quad(val: f32) -> f32 {
    1.0 - (1.0 - val) * (1.0 - val)
}

pub fn ease_in_out_quad(val: f32) -> f32 {
    if val < 0.5 {
        2.0 * val * val
    } else {
        1.0 - (-2.0 * val + 2.0).powf(2.0) / 2.0
    }
}

pub fn ease_in_cubic(val: f32) -> f32 {
    val * val * val
}

pub fn ease_out_cubic(val: f32) -> f32 {
    1.0 - (1.0 - val).powf(3.0)
}

pub fn ease_in_out_cubic(val: f32) -> f32 {
    if val < 0.5 {
        4.0 * val * val * val
    } else {
        1.0 - (-2.0 * val + 2.0).powf(3.0) / 2.0
    }
}

pub fn ease_in_quart(val: f32) -> f32 {
    val * val * val * val
}

pub fn ease_out_quart(val: f32) -> f32 {
    1.0 - (1.0 - val).powf(4.0)
}

pub fn ease_in_out_quart(val: f32) -> f32 {
    if val < 0.5 {
        8.0 * val * val * val * val
    } else {
        1.0 - (-2.0 * val + 2.0).powf(4.0) / 2.0
    }
}

pub fn ease_in_quint(val: f32) -> f32 {
    val * val * val * val * val
}

pub fn ease_out_quint(val: f32) -> f32 {
    1.0 - (1.0 - val).powf(5.0)
}

pub fn ease_in_out_quint(val: f32) -> f32 {
    if val < 0.5 {
        16.0 * val * val * val * val * val
    } else {
        1.0 - (-2.0 * val + 2.0).powf(5.0) / 2.0
    }
}

pub fn ease_in_expo(val: f32) -> f32 {
    if val == 0.0 {
        0.0
    } else {
        (2.0_f32).powf(10.0 * val - 10.0)
    }
}

pub fn ease_out_expo(val: f32) -> f32 {
    if val == 1.0 {
        1.0
    } else {
        1.0 - (2.0_f32).powf(-10.0 * val)
    }
}

pub fn ease_in_out_expo(val: f32) -> f32 {
    if val == 0.0 {
        0.0
    } else if val == 1.0 {
        1.0
    } else {
        if val < 0.5 {
            2.0_f32.powf(20.0 * val - 20.0) / 2.0
        } else {
            2.0 - 2.0_f32.powf(-20.0 * val - 20.0) / 2.0
        }
    }
}

pub fn ease_in_circ(val: f32) -> f32 {
    1.0 - (1.0 - val.powf(2.0)).sqrt()
}

pub fn ease_out_circ(val: f32) -> f32 {
    (1.0 - (val - 1.0).powf(2.0)).sqrt()
}

pub fn ease_in_out_circ(val: f32) -> f32 {
    if val < 0.5 {
        (1.0 - (1.0 - (2.0 * val).powf(2.0)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * val + 2.0).powf(2.0)) + 1.0).sqrt() / 2.0
    }
}

pub fn ease_in_back(val: f32) -> f32 {
    let c1: f32 = 1.70158;
    let c3 = c1 + 1.0;

    c3 * val * val * val - c1 * val * val
}

pub fn ease_out_back(val: f32) -> f32 {
    let c1: f32 = 1.70158;
    let c3 = c1 + 1.0;

    1.0 + c3 * (val - 1.0).powf(3.0) + c1 * (val - 1.0).powf(2.0)
}

pub fn ease_in_out_back(val: f32) -> f32 {
    let c1: f32 = 1.70158;
    let c2: f32 = c1 * 1.525;

    if val < 0.5 {
        ((2.0 * val).powf(2.0) * ((c2 + 1.0) * 2.0 * val - c2)) / 2.0
    } else {
        ((2.0 * val).powf(2.0) * ((c2 + 1.0) * 2.0 * val - c2)) / 2.0
    }
}

pub fn ease_in_elastic(val: f32) -> f32 {
    let c4 = (2.0 * std::f32::consts::PI) / 3.0;

    if val == 0.0 {
        0.0
    } else if val == 1.0 {
        1.0
    } else {
        -(2.0_f32).powf(10.0 * val * -10.0) * ((val * 10.0 - 10.75) * c4).sin()
    }
}

pub fn ease_out_elastic(val: f32) -> f32 {
    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
    if val == 0.0 {
        0.0
    } else if val == 1.0 {
        1.0
    } else {
        (2.0_f32).powf(-10.0 * val) * ((val * 10.0 - 0.75) * c4).sin() + 1.0
    }
}

pub fn ease_in_out_elastic(val: f32) -> f32 {
    let c5 = (2.0 * std::f32::consts::PI) / 4.5;
    if val == 0.0 {
        0.0
    } else if val == 1.0 {
        1.0
    } else if val < 0.5 {
        -(2.0_f32).powf(20.0 * val - 10.0) * ((20.0 * val - 11.125) * c5).sin() / 2.0
    } else {
        (2.0_f32).powf(-20.0 * val + 10.0) * ((20.0 * val - 11.125) * c5).sin() / 2.0
    }
}

pub fn ease_in_bounce(val: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - val)
}

pub fn ease_out_bounce(val: f32) -> f32 {
    let n1: f32 = 7.5625;
    let d1: f32 = 2.75;

    if val < 1.0 / d1 {
        n1 * val * val
    } else if val < 2.0 / d1 {
        n1 * (val - 1.5 / d1) * val + 0.75
    } else if val < 2.5 / d1 {
        n1 * (val - 2.25 / d1) * val + 0.9375
    } else {
        n1 * (val - 2.625 / d1) * val + 0.984375
    }
}

pub fn ease_in_out_bounce(val: f32) -> f32 {
    if val < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * val)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * val - 1.0)) / 2.0
    }
}
