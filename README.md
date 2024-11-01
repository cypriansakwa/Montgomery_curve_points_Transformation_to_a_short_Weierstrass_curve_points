# Transformation of Montgomery_curve_points_to_a_short_Weierstrass_curve_points

This Rust project implements the transformation from Montgomery curves to short Weierstrass curves using modular arithmetic. The code utilizes the `num-bigint` crate for handling large integers.

## Overview

The primary functionality includes:
- Defining structures for Montgomery and Weierstrass points.
- Performing modular arithmetic.
- Converting a point from a Montgomery curve to a Weierstrass curve.

## Dependencies

This project requires the following external crate:
- `num-bigint`

To add this dependency, include the following in your `Cargo.toml`:

>```
>[dependencies]
>num-bigint = "0.4"
