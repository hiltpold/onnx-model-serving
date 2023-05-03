use image::DynamicImage;
use image::{imageops::FilterType, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba};
use std::io::Cursor;

use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use onnxruntime::ndarray::Axis;
use onnxruntime::{
    environment::Environment, ndarray, tensor::OrtOwnedTensor, GraphOptimizationLevel, LoggingLevel,
};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type DetectionError = Box<dyn std::error::Error>;

pub struct ImageResult {
    pub result_image: Vec<u8>,
    pub bounding_boxes: Vec<BoundingBox>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub height: i32,
    pub width: i32,
    pub class: String,
    pub conf: f32,
}

pub fn detect(
    model_path: PathBuf,
    classes: &HashMap<usize, &str>,
    mut image: DynamicImage,
) -> Result<ImageResult, DetectionError> {
    let environment = Environment::builder()
        .with_name("dev")
        // The ONNX Runtime's log level can be different than the one of the wrapper crate or the application.
        .with_log_level(LoggingLevel::Info)
        .build()?;

    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file(model_path)
        .unwrap();

    let input0_shape: Vec<usize> = session.inputs[0].dimensions().map(|d| d.unwrap()).collect();
    let output0_shape: Vec<usize> = session.outputs[0]
        .dimensions()
        .map(|d| d.unwrap_or(0))
        .collect();

    assert_eq!(input0_shape, [1, 3, 640, 640]);
    assert_eq!(output0_shape, [0, 7]);

    // Load image

    // Convert to RGB format
    let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = image
        .resize_exact(
            input0_shape[2] as u32,
            input0_shape[3] as u32,
            FilterType::Nearest,
        )
        .to_rgb8();

    println!("Dimensions image: {:?}", image.dimensions());

    let array = ndarray::Array::from_shape_fn((1, 3, 640, 640), |(_, c, j, i)| {
        let pixel = image_buffer.get_pixel(i as u32, j as u32);
        let channels = pixel.channels();

        // range [0, 255] -> range [0, 1]
        (channels[c] as f32) / 255.0
    });

    let input_tensor_values = vec![array];

    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor_values)?;
    let output: &OrtOwnedTensor<f32, _> = &outputs[0];

    let mut bounding_boxes = Vec::new();

    // returns width height from image
    let image_shape = image.dimensions();
    output.axis_iter(Axis(0)).for_each(|bb| {
        // calculate image gain old_(width/height) / resized_/(width/height)
        let gain_x = image_shape.0 as f32 / 640.0;
        let gain_y = image_shape.1 as f32 / 640.0;

        let bounding_box = bb.as_slice().unwrap();

        let min_x = (bounding_box[1] * gain_x).round() as i32;
        let min_y = (bounding_box[2] * gain_y).round() as i32;
        let max_x = (bounding_box[3] * gain_x).round() as i32;
        let max_y = (bounding_box[4] * gain_y).round() as i32;

        let width = max_x - min_x;
        let height = max_y - min_y;

        println!("Width: {:?}", width);
        println!("Height: {:?}", height);

        assert!(width > 0);
        assert!(height > 0);

        let rect = Rect::at(min_x, min_y).of_size((width) as u32, (height) as u32);

        let green = Rgba([0u8, 255u8, 0u8, 255u8]);
        let white = Rgba([255u8, 255u8, 255u8, 255u8]);

        let font = Vec::from(include_bytes!("./font/courier_new.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        let text = classes
            .get(&(bounding_box[5] as usize))
            .copied()
            .unwrap()
            .to_owned()
            + "|"
            + format!("{:.2}", bounding_box[6]).as_str();
        let font_size = 25.0;
        let scale = Scale {
            x: font_size,
            y: font_size,
        };
        draw_hollow_rect_mut(&mut image, rect, green);
        draw_text_mut(&mut image, white, min_x, min_y, scale, &font, &text);

        bounding_boxes.push(BoundingBox {
            min_x: min_x,
            min_y: min_y,
            height: height,
            width: width,
            class: classes
                .get(&(bounding_box[5] as usize))
                .copied()
                .unwrap()
                .to_string(),
            conf: bounding_box[6],
        })
    });

    //image.save(file_path_out).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    image.write_to(
        &mut Cursor::new(&mut buf),
        image::ImageOutputFormat::Jpeg(255u8),
    )?;

    Ok(ImageResult {
        result_image: buf,
        bounding_boxes: bounding_boxes,
    })
}
