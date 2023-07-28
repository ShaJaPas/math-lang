pub struct Op;

const EPS: f32 = 0.0000001;

impl Op {
    pub fn add(lhs: f32, rhs: f32) -> f32 {
        lhs + rhs
    }

    pub fn substract(lhs: f32, rhs: f32) -> f32 {
        lhs - rhs
    }

    pub fn multiply(lhs: f32, rhs: f32) -> f32 {
        lhs * rhs
    }

    pub fn divide(lhs: f32, rhs: f32) -> Result<f32, String> {
        if rhs == 0.0 {
            Err("Division by zero!".to_owned())
        } else {
            Ok(lhs / rhs)
        }
    }

    pub fn power(lhs: f32, rhs: f32) -> f32 {
        lhs.powf(rhs)
    }

    pub fn modulus(lhs: f32, rhs: f32) -> Result<f32, String> {
        if rhs == 0.0 {
            Err("Modulus division by zero!".to_owned())
        } else {
            let res = lhs.rem_euclid(rhs);
            Ok(if (res - rhs).abs() <= EPS { 0.0 } else { res })
        }
    }

    pub fn div(lhs: f32, rhs: f32) -> Result<f32, String> {
        if rhs == 0.0 {
            Err("Integer division by zero!".to_owned())
        } else {
            Ok(lhs.div_euclid(rhs))
        }
    }

    pub fn unary_minus(rhs: f32) -> f32 {
        -rhs
    }

    pub fn factorial(rhs: f32) -> Result<f32, String> {
        let rhs = rhs as i32;
        match rhs {
            0 => Ok(1.0),
            _ if rhs < 0 => Err("Cannot get factorial from negative integer!".to_owned()),
            rhs => Ok((1..=rhs).fold(1.0, |acc, x| acc * x as f32)),
        }
    }

    pub fn sin(rhs: f32) -> f32 {
        rhs.sin()
    }

    pub fn cos(rhs: f32) -> f32 {
        rhs.cos()
    }

    pub fn tg(rhs: f32) -> f32 {
        rhs.tan()
    }

    pub fn ctg(rhs: f32) -> f32 {
        1.0 / rhs.tan()
    }

    pub fn log(value: f32, base: f32) -> Result<f32, String> {
        if base <= 0.0 || base == 1.0 {
            Err("Log base must be positive and not equals to 1".to_owned())
        } else if value <= 0.0 {
            Err("Log value must be positive".to_owned())
        } else {
            Ok(value.log(base))
        }
    }
}
