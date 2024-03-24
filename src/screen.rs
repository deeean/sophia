use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::geometry::Point;

#[napi]
#[derive(Debug, Clone)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pixel_width: u8,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[napi]
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
};

#[napi]
pub async fn read_image_data(path: String) -> Result<ImageData> {
    match tokio::spawn(async move {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => return Err(e),
        };

        let width = img.width();
        let height = img.height();
        let pixel_width = img.color().bytes_per_pixel();
        let data = img.as_bytes().to_vec();

        Ok(ImageData {
            data,
            width,
            height,
            pixel_width,
        })
    }).await {
        Ok(data) => {
            match data {
                Ok(data) => Ok(data),
                Err(e) => Err(Error::new(
                    Status::GenericFailure,
                    format!("Error: {:?}", e),
                )),
            }
        },
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn write_image_data(path: String, image_data: &ImageData) -> Result<()> {
    let path = path.clone();
    let image_data = image_data.clone();

    match tokio::spawn(async move {
        let image_buffer =
            match image::RgbaImage::from_raw(image_data.width, image_data.height, image_data.data) {
                Some(buffer) => buffer,
                None => return Err(Error::new(
                    Status::GenericFailure,
                    "Failed to create image buffer",
                )),
            };

        match image::DynamicImage::ImageRgba8(image_buffer).save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }).await {
        Ok(res) => match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn image_search(
    source: &ImageData,
    target: &ImageData,
    variant: Option<i32>,
    trans_color: Option<Color>,
) -> Result<Option<Point>> {
    let variant = variant.unwrap_or(0);
    let source = source.clone();
    let target = target.clone();

    match tokio::spawn(async move {
        if let Some(trans_color) = trans_color {
            image_search_trans_inner(&source, &target, variant, trans_color)
        } else {
            image_search_inner(&source, &target, variant)
        }
    }).await {
        Ok(res) => Ok(res),
        Err(e) => {
            Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            ))
        },
    }
}

#[napi]
pub async fn multiple_image_search(
    source: &ImageData,
    target: &ImageData,
    variant: Option<i32>,
    trans_color: Option<Color>,
) -> Result<Vec<Point>> {
    let variant = variant.unwrap_or(0);
    let source = source.clone();
    let target = target.clone();

    match tokio::spawn(async move {
        if let Some(trans_color) = trans_color {
            multiple_image_search_trans_inner(&source, &target, variant, trans_color)
        } else {
            multiple_image_search_inner(&source, &target, variant)
        }
    }).await {
        Ok(res) => Ok(res),
        Err(e) => {
            Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            ))
        },
    }
}

fn multiple_image_search_inner(
    source: &ImageData,
    target: &ImageData,
    variant: i32,
) -> Vec<Point> {
    let source_pixels = source.data.as_slice();
    let target_pixels = target.data.as_slice();

    let source_width = source.width;
    let source_height = source.height;

    let target_width = target.width;
    let target_height = target.height;

    let source_pixel_width = source.pixel_width as u32;
    let target_pixel_width = target.pixel_width as u32;

    let source_pixel_count = source_width * source_height;
    let target_pixel_count = target_width * target_height;
    let mut points = Vec::new();

    if variant == 0 {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index];
                let source_green = source_pixels[source_index + 1];
                let source_blue = source_pixels[source_index + 2];

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index];
                let green = target_pixels[target_index + 1];
                let blue = target_pixels[target_index + 2];

                is_found = source_red == red && source_green == green && source_blue == blue;

                if !is_found {
                    break;
                }
            }

            if is_found {
                points.push(Point { x: sx as i32, y: sy as i32 });
            }
        }
    } else {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index] as i32;
                let source_green = source_pixels[source_index + 1] as i32;
                let source_blue = source_pixels[source_index + 2] as i32;

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index] as i32;
                let green = target_pixels[target_index + 1] as i32;
                let blue = target_pixels[target_index + 2] as i32;

                let red_low = if source_red < variant {
                    0
                } else {
                    source_red - variant
                };
                let red_high = if source_red + variant > 255 {
                    255
                } else {
                    source_red + variant
                };

                let green_low = if source_green < variant {
                    0
                } else {
                    source_green - variant
                };
                let green_high = if source_green + variant > 255 {
                    255
                } else {
                    source_green + variant
                };

                let blue_low = if source_blue < variant {
                    0
                } else {
                    source_blue - variant
                };
                let blue_high = if source_blue + variant > 255 {
                    255
                } else {
                    source_blue + variant
                };

                is_found = red >= red_low
                    && red <= red_high
                    && green >= green_low
                    && green <= green_high
                    && blue >= blue_low
                    && blue <= blue_high;

                if !is_found {
                    break;
                }
            }

            if is_found {
                points.push(Point {
                    x: sx as i32,
                    y: sy as i32
                });
            }
        }
    }

    points
}

fn image_search_inner(
    source: &ImageData,
    target: &ImageData,
    variant: i32,
) -> Option<Point> {
    let source_pixels = source.data.as_slice();
    let target_pixels = target.data.as_slice();

    let source_width = source.width;
    let source_height = source.height;

    let target_width = target.width;
    let target_height = target.height;

    let source_pixel_width = source.pixel_width as u32;
    let target_pixel_width = target.pixel_width as u32;

    let source_pixel_count = source_width * source_height;
    let target_pixel_count = target_width * target_height;

    if variant == 0 {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index];
                let source_green = source_pixels[source_index + 1];
                let source_blue = source_pixels[source_index + 2];

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index];
                let green = target_pixels[target_index + 1];
                let blue = target_pixels[target_index + 2];

                is_found = source_red == red && source_green == green && source_blue == blue;

                if !is_found {
                    break;
                }
            }

            if is_found {
                return Some(Point { x: sx as i32, y: sy as i32 });
            }
        }
    } else {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index] as i32;
                let source_green = source_pixels[source_index + 1] as i32;
                let source_blue = source_pixels[source_index + 2] as i32;

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index] as i32;
                let green = target_pixels[target_index + 1] as i32;
                let blue = target_pixels[target_index + 2] as i32;

                let red_low = if source_red < variant {
                    0
                } else {
                    source_red - variant
                };
                let red_high = if source_red + variant > 255 {
                    255
                } else {
                    source_red + variant
                };

                let green_low = if source_green < variant {
                    0
                } else {
                    source_green - variant
                };
                let green_high = if source_green + variant > 255 {
                    255
                } else {
                    source_green + variant
                };

                let blue_low = if source_blue < variant {
                    0
                } else {
                    source_blue - variant
                };
                let blue_high = if source_blue + variant > 255 {
                    255
                } else {
                    source_blue + variant
                };

                is_found = red >= red_low
                    && red <= red_high
                    && green >= green_low
                    && green <= green_high
                    && blue >= blue_low
                    && blue <= blue_high;

                if !is_found {
                    break;
                }
            }

            if is_found {
                return Some(Point { x: sx as i32, y: sy as i32 });
            }
        }
    }

    None
}

fn image_search_trans_inner(
    source: &ImageData,
    target: &ImageData,
    variant: i32,
    trans_color: Color,
) -> Option<Point> {
    let source_pixels = source.data.as_slice();
    let target_pixels = target.data.as_slice();

    let source_width = source.width;
    let source_height = source.height;

    let target_width = target.width;
    let target_height = target.height;

    let source_pixel_width = source.pixel_width as u32;
    let target_pixel_width = target.pixel_width as u32;

    let source_pixel_count = source_width * source_height;
    let target_pixel_count = target_width * target_height;

    if variant == 0 {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index];
                let source_green = source_pixels[source_index + 1];
                let source_blue = source_pixels[source_index + 2];

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index];
                let green = target_pixels[target_index + 1];
                let blue = target_pixels[target_index + 2];

                is_found = (trans_color.r == red && trans_color.g == green && trans_color.b == blue)
                    || (source_red == red && source_green == green && source_blue == blue);

                if !is_found {
                    break;
                }
            }

            if is_found {
                return Some(Point { x: sx  as i32, y: sy  as i32 });
            }
        }
    } else {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index] as i32;
                let source_green = source_pixels[source_index + 1] as i32;
                let source_blue = source_pixels[source_index + 2] as i32;

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index] as i32;
                let green = target_pixels[target_index + 1] as i32;
                let blue = target_pixels[target_index + 2] as i32;

                if trans_color.r == red as u8 && trans_color.g == green as u8 && trans_color.b == blue as u8 {
                    continue;
                }

                let red_low = if source_red < variant {
                    0
                } else {
                    source_red - variant
                };
                let red_high = if source_red + variant > 255 {
                    255
                } else {
                    source_red + variant
                };

                let green_low = if source_green < variant {
                    0
                } else {
                    source_green - variant
                };
                let green_high = if source_green + variant > 255 {
                    255
                } else {
                    source_green + variant
                };

                let blue_low = if source_blue < variant {
                    0
                } else {
                    source_blue - variant
                };
                let blue_high = if source_blue + variant > 255 {
                    255
                } else {
                    source_blue + variant
                };

                is_found = red >= red_low
                    && red <= red_high
                    && green >= green_low
                    && green <= green_high
                    && blue >= blue_low
                    && blue <= blue_high;

                if !is_found {
                    break;
                }
            }

            if is_found {
                return Some(Point { x: sx as i32, y: sy as i32 });
            }
        }
    }

    None
}

fn multiple_image_search_trans_inner(
    source: &ImageData,
    target: &ImageData,
    variant: i32,
    trans: Color,
) -> Vec<Point> {
    let source_pixels = source.data.as_slice();
    let target_pixels = target.data.as_slice();

    let source_width = source.width;
    let source_height = source.height;

    let target_width = target.width;
    let target_height = target.height;

    let source_pixel_width = source.pixel_width as u32;
    let target_pixel_width = target.pixel_width as u32;

    let source_pixel_count = source_width * source_height;
    let target_pixel_count = target_width * target_height;
    let mut points = Vec::new();

    if variant == 0 {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index];
                let source_green = source_pixels[source_index + 1];
                let source_blue = source_pixels[source_index + 2];

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index];
                let green = target_pixels[target_index + 1];
                let blue = target_pixels[target_index + 2];

                is_found = (trans.r == red && trans.g == green && trans.b == blue)
                    || source_red == red && source_green == green && source_blue == blue;

                if !is_found {
                    break;
                }
            }

            if is_found {
                points.push(Point { x: sx as i32, y: sy as i32 });
            }
        }
    } else {
        for i in 0..source_pixel_count {
            let sx = i % source_width;
            let sy = i / source_width;

            if sx + target_width > source_width || sy + target_height > source_height {
                continue;
            }

            let mut is_found = true;

            for j in 0..target_pixel_count {
                let tx = j % target_width;
                let ty = j / target_width;

                let x = sx + tx;
                let y = sy + ty;

                let source_index = ((y * source_width + x) * source_pixel_width) as usize;
                let source_red = source_pixels[source_index] as i32;
                let source_green = source_pixels[source_index + 1] as i32;
                let source_blue = source_pixels[source_index + 2] as i32;

                let target_index = (j * target_pixel_width) as usize;

                let red = target_pixels[target_index] as i32;
                let green = target_pixels[target_index + 1] as i32;
                let blue = target_pixels[target_index + 2] as i32;

                if trans.r == red as u8 && trans.g == green as u8 && trans.b == blue as u8 {
                    continue;
                }

                let red_low = if source_red < variant {
                    0
                } else {
                    source_red - variant
                };
                let red_high = if source_red + variant > 255 {
                    255
                } else {
                    source_red + variant
                };

                let green_low = if source_green < variant {
                    0
                } else {
                    source_green - variant
                };
                let green_high = if source_green + variant > 255 {
                    255
                } else {
                    source_green + variant
                };

                let blue_low = if source_blue < variant {
                    0
                } else {
                    source_blue - variant
                };
                let blue_high = if source_blue + variant > 255 {
                    255
                } else {
                    source_blue + variant
                };

                is_found = red >= red_low
                    && red <= red_high
                    && green >= green_low
                    && green <= green_high
                    && blue >= blue_low
                    && blue <= blue_high;

                if !is_found {
                    break;
                }
            }

            if is_found {
                points.push(Point { x: sx as i32, y: sy as i32 });
            }
        }
    }

    points
}