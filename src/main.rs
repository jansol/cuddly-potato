/**************************************************************************
 * Cuddly Potato - An image generator with a REST API
 * Copyright (C) 2017  Jan Solanti
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *************************************************************************/

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::ContentType;
use rocket::response::content::{Content, Html};

extern crate png;
use png::HasParameters;

mod palettes;

#[get("/favicon.ico")]
fn favicon() -> Content<Vec<u8>> {
    let bytes = include_bytes!("favicon.ico");
    Content(ContentType::new("image", "x-icon"), Vec::from(&bytes[..]))
}

#[get("/", format = "text/html")]
fn index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
    <head>
        <link rel="icon" href="/favicon.ico" />
        <title>△ Fractals △</title>
    </head>
    <body>
        <p>
            <a href="mandelbrot?center_x=-0.75&center_y=0&width=1024&height=1024&scale=0.35&palette_scale=3">
                mandelbrot example
            </a>
        </p>

        <p>
            <a href="mandelbrot?center_x=0.0016&center_y=-0.8225&width=512&height=512&scale=3200">
                another mandelbrot example
            </a>
        </p>

        <p>
            <a href="checkerboard?center_x=0.5&center_y=-0.333&width=512&height=512&scale=64">
                checkerboard
            </a>
        </p>
    </body>
</html>
    "#)
}

#[derive(FromForm, Debug)]
struct Camera {
    center_x: f64,
    center_y: f64,
    width: u32,
    height: u32,
    scale: f64,
    palette_scale: Option<f64>
}

fn png_response(data: &Vec<u8>, width: u32, height: u32) -> Content<Vec<u8>> {
    let mut buf = Vec::new();
    {
        let mut e = png::Encoder::new(&mut buf, width, height);
        e.set(png::ColorType::RGB)
         .set(png::BitDepth::Eight);
        e.write_header().unwrap()
         .write_image_data(data).unwrap();
    }

    Content(ContentType::PNG, buf)
}

fn palette_lookup(mu: f64) -> (u8,u8,u8) {
    use palettes;
    let palette = palettes::DEFAULT;

    fn degamma(b: u8) -> f64 { (b as f64 / 255f64).powf(2.2f64) }
    fn gamma(f: f64) -> u8 { (f.powf(1f64/2.2f64) * 255f64) as u8 }

    if !mu.is_finite() {
        return palette[palette.len()-1];
    }

    let idx_max = (palette.len()-1).max(0);
    let mu = mu / idx_max as f64;

    let a = palette[mu.floor().max(0f64).min(idx_max as f64) as usize];
    let b = palette[mu.ceil().max(0f64).min(idx_max as f64) as usize];
    let t = mu.fract();

    let a = (degamma(a.0),degamma(a.1),degamma(a.2));
    let b = (degamma(b.0),degamma(b.1),degamma(b.2));

    (
        gamma(t*b.0 + (1f64-t)*a.0),
        gamma(t*b.1 + (1f64-t)*a.1),
        gamma(t*b.2 + (1f64-t)*a.2),
    )
}

#[get("/checkerboard?<camera>")]
fn checkerboard(camera: Camera) -> Content<Vec<u8>> {
    const LIGHT: u8 = 0xA9;
    const DARK: u8 = 0x55;
    let square_size = camera.scale.max(0.00001);
    let mut pixels = Vec::with_capacity(3usize * camera.width as usize * camera.height as usize);

    let x_shift = (camera.center_x - camera.width as f64).abs() * camera.scale;
    let y_shift = (camera.center_y - camera.height as f64).abs() * camera.scale;

    for y in 0..camera.height {
        for x in 0..camera.width {
            let x0 = x as f64 + x_shift;
            let y0 = y as f64 + y_shift;

            if (x0 % (2f64*square_size) < square_size) ^ (y0 % (2f64*square_size) < square_size) {
                pixels.push(LIGHT);
                pixels.push(LIGHT);
                pixels.push(LIGHT);
            } else {
                pixels.push(DARK);
                pixels.push(DARK);
                pixels.push(DARK);
            }
        }
    }

    png_response(&pixels, camera.width, camera.height)
}

#[get("/mandelbrot?<camera>")]
fn mandelbrot(camera: Camera) -> Content<Vec<u8>> {
    const ITERATION_MAX: i64 = 100;
    const ESCAPE_RADIUS: f64 = 1e75;

    let aspect = camera.width as f64/camera.height as f64;
    let mut pixels = Vec::with_capacity(3usize * camera.width as usize * camera.height as usize);
    let w = camera.width as i64;
    let h = camera.height as i64;

    for y in -h/2..(h as f64/2f64).round() as i64 {
        for x in -w/2..(w as f64/2f64).round() as i64 {
            let re_0 = camera.center_x + (x as f64/w as f64)*aspect / camera.scale;
            let im_0 = camera.center_y + (y as f64/h as f64) / camera.scale;
            let mut re_z = 0f64;
            let mut im_z = 0f64;
            let mut iteration = 0i64;
            let mut modulus = 0f64;

            while modulus < ESCAPE_RADIUS as f64 && iteration < ITERATION_MAX {
                let re_temp = re_z*re_z - im_z*im_z + re_0;
                im_z = 2f64*re_z*im_z + im_0;
                re_z = re_temp;
                modulus = (re_z*re_z + im_z*im_z).sqrt();
                iteration += 1;
            }
            let mu = iteration as f64 - modulus.log10().log10() / 2f64.log10();

            let col = palette_lookup(mu * camera.palette_scale.unwrap_or(1f64));
            pixels.push(col.0);
            pixels.push(col.1);
            pixels.push(col.2);
        }
    }

    png_response(&pixels, camera.width, camera.height)
}

fn main() {
    rocket::ignite()
        //.manage(Cache<Camera, Vec<u8>>::new(128))
        .mount("/", routes![index, favicon, checkerboard, mandelbrot])
        .launch();
}
