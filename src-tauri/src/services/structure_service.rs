use anyhow::{anyhow, Result};
use std::{
    char::MAX,
    collections::{HashMap, HashSet},
    iter::{self, zip},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::State;

use serde::__private::de;

use crate::{
    dto::{
        spline_dto::{SplineDtoRaw, SplineDtoWithDeriv},
        structure_dto::StructureDto,
    },
    models::core::{
        spline::Spline,
        structure::{Metric, Structure},
    },
    state::AppState,
    transformers::path_buf_transformer,
    utils::{asserts::assert_result_msg, math::relative_eq},
};

use super::csv_service;

const LINE_SPLITTER: &str = "\n";
const CELL_SPLITTER: &str = ",";
const X_VALUE_LINE_INDEX: usize = 4;
const Y_VALUE_LINE_INDEX: usize = 5;
const CELL_SUFFIX: &str = "\r";

pub fn create_structure(path_buf: PathBuf) -> Result<Structure> {
    let csv_string = csv_service::read_csv(&path_buf)?;
    let file_name = path_buf_transformer::to_file_name(&path_buf)?;
    let structure_dto = create_structure_dto(file_name, csv_string)?;
    println!("Created StructureDTO Sucessful");
    create_structure_from_dto(structure_dto)
}

fn get_metric(file_name: &str) -> Result<Metric> {
    let metric_map = HashMap::from([
        ("cd", Metric::CurrentDensity),
        ("cdvh", Metric::CurrentDensity),
        ("e-field", Metric::ElectricField),
        ("evh", Metric::ElectricField),
        ("sar", Metric::SpecificAbsorbanceRate),
        ("sarvh", Metric::SpecificAbsorbanceRate),
    ]);

    let metric_str = file_name.split(" ").next().ok_or(anyhow!(
        "Could not get metric name from file name {}",
        file_name
    ))?;
    let metric_str_lower: &str = &metric_str.to_ascii_lowercase();

    metric_map
        .get(metric_str_lower)
        .ok_or(anyhow!("Metric name of {} not found", metric_str))
        .map(Metric::to_owned)
}

fn get_name(file_name: &str) -> String {
    let file_name_parts: Vec<String> = file_name
        .split(" ")
        .into_iter()
        .skip(1)
        .filter(|file_name_part| !file_name_part.to_ascii_lowercase().eq("raw.csv"))
        .map(str::to_owned)
        .collect();

    file_name_parts.join(" ")
}

pub fn create_structure_dto(file_name: String, csv_string: String) -> Result<StructureDto> {
    let lines: Vec<&str> = csv_string
        .split(LINE_SPLITTER)
        .into_iter()
        .filter(|line| is_valid_csv_line(line))
        .collect();

    assert_result_msg(
        6,
        lines.len(),
        format!("csv file must contain 6 lines but contains {}", lines.len()),
    )?;

    let x_values_line = lines
        .get(X_VALUE_LINE_INDEX)
        .ok_or(anyhow!("Index out of bounds"))?;
    let y_values_line = lines
        .get(Y_VALUE_LINE_INDEX)
        .ok_or(anyhow!("Index out of bounds"))?;

    let mut points_iter = zip(
        x_values_line.split(CELL_SPLITTER).into_iter(),
        y_values_line.split(CELL_SPLITTER).into_iter(),
    )
    .into_iter();

    // The first column stores the frequency
    let (_, frequency_str) = points_iter
        .next()
        .ok_or(anyhow!("Frequency is not defined"))?;

    let frequency: f32 = frequency_str.trim().parse()?;

    let points: Result<Vec<(f32, f32)>> = points_iter
        .map(|(x_cell_str, y_cell_str)| {
            let y: f32 = parse_y_str(y_cell_str)?;
            let x: f32 = parse_x_str(x_cell_str)?;
            Ok((x, y))
        })
        .collect();
    let points = points?;

    let mut spline_dtos = Vec::new();
    for (i, _point) in points.iter().enumerate() {
        if let (Some((x1, y1)), Some((x2, y2))) = (points.get(i), points.get(i + 1)) {
            let spline_dto_raw =
                SplineDtoRaw::new(x1.to_owned(), y1.to_owned(), x2.to_owned(), y2.to_owned());
            spline_dtos.push(spline_dto_raw);
        }
    }

    let metric = get_metric(&file_name)?;
    let name = get_name(&file_name);

    let structure_dto = StructureDto {
        name: name,
        frequency: frequency,
        file_name: file_name,
        metric: metric,
        splines: spline_dtos,
    };
    Ok(structure_dto)
}

fn is_valid_csv_line(line: &str) -> bool {
    let is_invalid_line = line.is_empty() || line.eq("\r");
    !is_invalid_line
}

fn parse_x_str(x_cell_str: &str) -> Result<f32> {
    let x_str_end = x_cell_str
        .split(|c| c == '<' || c == '>' || c == '=')
        .into_iter()
        .filter(|str| !str.trim().is_empty())
        .nth(1)
        .ok_or(anyhow!("Cannot parse {} into a value", x_cell_str))?;

    let x_str = x_str_end.split("(cm^3)").into_iter().next().ok_or(anyhow!(
        "Cannot parse cell value {} into a value",
        x_str_end
    ))?;
    let x = x_str
        .trim()
        .parse::<f32>()
        .map_err(|e| anyhow!("Invalid float literal for {}", x_str))?;
    Ok(x)
}

fn parse_y_str(y_cell_str: &str) -> Result<f32> {
    let y_str = match y_cell_str.strip_suffix(CELL_SUFFIX) {
        Some(y_str) => y_str,
        None => y_cell_str,
    };
    let y = y_str
        .trim()
        .parse::<f32>()
        .map_err(|e| anyhow!("{} for {}", e.to_string(), y_cell_str))?;
    Ok(y)
}

pub fn create_structure_from_dto(structure_dto: StructureDto) -> Result<Structure> {
    let raw_splines = structure_dto.splines;
    let deltas = raw_splines.iter().map(convert_raw_to_with_delta).collect();
    let splines_with_initial_guess = get_derivative_guesses(deltas);
    let derivatives = splines_with_initial_guess
        .iter()
        .map(map_guess_to_f2_area)
        .collect();
    let derivatives = set_consistent_derivatives(derivatives);
    let splines = raw_splines
        .iter()
        .zip(derivatives.iter())
        .map(|(raw, deriv)| spline_dto_to_spline(raw, deriv))
        .collect();

    Ok(Structure::new(
        structure_dto.name,
        structure_dto.file_name,
        structure_dto.metric,
        splines,
    ))
}

fn convert_raw_to_with_delta(spline_raw: &SplineDtoRaw) -> f32 {
    (spline_raw.y2 - spline_raw.y1) / (spline_raw.x2 - spline_raw.x1)
}

fn get_derivative_guesses(deltas: Vec<f32>) -> Vec<SplineDtoWithDeriv> {
    let n = deltas.len();
    let mut derivative_guesses: Vec<Vec<f32>> = iter::repeat(vec![0.0; 2]).take(n).collect();

    derivative_guesses[0][0] = deltas[0] * 0.5;
    derivative_guesses[n - 1][1] = deltas[n - 1] * 0.5;

    for (index, spline_dto) in deltas.iter().take(n - 1).skip(1).enumerate() {
        let d = (deltas[index] + deltas[index + 1]) * 0.5;
        derivative_guesses[index][1] = d;
        derivative_guesses[index + 1][0] = d;
    }

    deltas
        .iter()
        .zip(derivative_guesses.iter())
        .map(|(delta, derivative_guess)| SplineDtoWithDeriv {
            delta: *delta,
            left_deriv: derivative_guess[0],
            right_deriv: derivative_guess[1],
        })
        .collect()
}

fn map_guess_to_f2_area(initial_guess: &SplineDtoWithDeriv) -> SplineDtoWithDeriv {
    let alpha;
    let beta;
    const MAX_RADIUS: f32 = 9.0;

    if relative_eq(initial_guess.delta, 0.0) {
        alpha = 0.0;
        beta = 0.0;
    } else {
        alpha = initial_guess.left_deriv / initial_guess.delta;
        beta = initial_guess.right_deriv / initial_guess.delta;
    }

    let mut left_deriv = initial_guess.left_deriv;
    let mut right_deriv = initial_guess.right_deriv;

    let solution_radius: f32 = alpha.powi(2) + beta.powi(2);

    if solution_radius > MAX_RADIUS {
        left_deriv = (3.0 * initial_guess.delta * alpha) / solution_radius.sqrt();
        right_deriv = (3.0 * initial_guess.delta * beta) / solution_radius.sqrt();
    }

    SplineDtoWithDeriv {
        delta: initial_guess.delta,
        left_deriv: left_deriv,
        right_deriv: right_deriv,
    }
}

fn set_consistent_derivatives(splines: Vec<SplineDtoWithDeriv>) -> Vec<SplineDtoWithDeriv> {
    let n = splines.len();
    let mut final_derivatives = splines.clone();
    for (index, (left_spline, right_spline)) in splines
        .iter()
        .take(n - 1)
        .zip(splines.iter().skip(1))
        .enumerate()
    {
        if left_spline.right_deriv.abs() < right_spline.left_deriv.abs() {
            final_derivatives[index + 1].left_deriv = left_spline.right_deriv;
        } else {
            final_derivatives[index].right_deriv = right_spline.left_deriv;
        }
    }
    final_derivatives
}

fn spline_dto_to_spline(
    spline_dto_raw: &SplineDtoRaw,
    spline_dto_deriv: &SplineDtoWithDeriv,
) -> Spline {
    let delta = spline_dto_deriv.delta;
    let h = spline_dto_raw.x2 - spline_dto_raw.x1;

    let a = (spline_dto_deriv.left_deriv + spline_dto_deriv.right_deriv - 2. * delta) / h.powi(2);
    let a = (spline_dto_deriv.left_deriv + spline_dto_deriv.right_deriv - 2. * delta) / h.powi(2);
    let b = (-2. * spline_dto_deriv.left_deriv - spline_dto_deriv.right_deriv + 3. * delta) / h;
    let c = spline_dto_deriv.left_deriv;
    let d = spline_dto_raw.y1;

    let i = a;
    let j = -3. * a * spline_dto_raw.x1 + b;
    let k = 3. * a * spline_dto_raw.x1.powi(2) - 2. * b * spline_dto_raw.x1 + c;
    let l =
        -a * spline_dto_raw.x1.powi(3) + b * spline_dto_raw.x1.powi(2) - c * spline_dto_raw.x1 + d;

    let point1 = (spline_dto_raw.x1, spline_dto_raw.y1);
    let point2 = (spline_dto_raw.x2, spline_dto_raw.y2);
    let coefficients = (i, j, k, l);

    Spline::new(
        spline_dto_raw.x1,
        spline_dto_raw.y1,
        spline_dto_raw.x2,
        spline_dto_raw.y2,
        a,
        b,
        c,
        d,
    )
}

#[cfg(test)]
mod tests {
    use crate::dto::structure_dto;

    use super::*;

    #[test]
    fn test_create_structure_dto() {
        let file_name = "E-field Brainstem Raw.csv".to_string();
        let csv_string = "% Model,EY106 BRAIN 03022021 Case G3.mph
            % Version,COMSOL 6.1.0.252
            % Date,\"Aug 10 2023, 23:18\"
            % Table,Table: E-field Brainstem {tbl12} - 
            % freq (Hz),ec.normE/sqrt(2)>=0 (cm^3),ec.normE/sqrt(2)>0.01 (cm^3),ec.normE/sqrt(2)>0.5 (cm^3),ec.normE/sqrt(2)>1 (cm^3),ec.normE/sqrt(2)>2 (cm^3),ec.normE/sqrt(2)>3 (cm^3),ec.normE/sqrt(2)>4 (cm^3),ec.normE/sqrt(2)>5 (cm^3),ec.normE/sqrt(2)>6 (cm^3),ec.normE/sqrt(2)>7 (cm^3),ec.normE/sqrt(2)>8 (cm^3),ec.normE/sqrt(2)>9 (cm^3),ec.normE/sqrt(2)>10 (cm^3),ec.normE/sqrt(2)>20 (cm^3),ec.normE/sqrt(2)>30 (cm^3),ec.normE/sqrt(2)>40 (cm^3),ec.normE/sqrt(2)>50 (cm^3),ec.normE/sqrt(2)>60 (cm^3),ec.normE/sqrt(2)>70 (cm^3),ec.normE/sqrt(2)>80 (cm^3),ec.normE/sqrt(2)>90 (cm^3),ec.normE/sqrt(2)>100 (cm^3),ec.normE/sqrt(2)>125 (cm^3),ec.normE/sqrt(2)>150 (cm^3),ec.normE/sqrt(2)>175 (cm^3),ec.normE/sqrt(2)>200 (cm^3),ec.normE/sqrt(2)>300 (cm^3),ec.normE/sqrt(2)>400 (cm^3),ec.normE/sqrt(2)>500 (cm^3),ec.normE/sqrt(2)>1000 (cm^3),ec.normE/sqrt(2)>2000 (cm^3),ec.normE/sqrt(2)>3000 (cm^3),ec.normE/sqrt(2)>4000 (cm^3),ec.normE/sqrt(2)>5000 (cm^3),ec.normE/sqrt(2)>6000 (cm^3),ec.normE/sqrt(2)>7000 (cm^3),ec.normE/sqrt(2)>8000 (cm^3),ec.normE/sqrt(2)>9000 (cm^3),ec.normE/sqrt(2)>10000 (cm^3),ec.normE/sqrt(2)>15000 (cm^3),ec.normE/sqrt(2)>20000 (cm^3)
            200000,31.427148660832184,31.427148660832184,31.427148660832184,31.427148660832184,31.427148660832184,31.427148660832184,31.427141568072287,31.42691361008912,31.42600382558215,31.421793059572494,31.408318589010868,31.384975012692777,31.344128541785295,17.580095981454612,1.9046295646341747,0.25105171146401645,0.04937469717986531,0.010076994504525627,0.002483197463210391,9.175030754800346E-4,3.499001335984863E-4,1.547624843804349E-4,1.6307125042320365E-5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".to_string();

        let structure_dto = create_structure_dto(file_name, csv_string).unwrap();

        assert_eq!(200000., structure_dto.frequency);
        let expected_y: Vec<f32> = vec![
            31.427148660832184,
            31.427148660832184,
            31.427148660832184,
            31.427148660832184,
            31.427148660832184,
            31.427148660832184,
            31.427141568072287,
            31.42691361008912,
            31.42600382558215,
            31.421793059572494,
            31.408318589010868,
            31.384975012692777,
            31.344128541785295,
            17.580095981454612,
            1.9046295646341747,
            0.25105171146401645,
            0.04937469717986531,
            0.010076994504525627,
            0.002483197463210391,
            9.175030754800346E-4,
            3.499001335984863E-4,
            1.547624843804349E-4,
            1.6307125042320365E-5,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        let actual_y: Vec<f32> = structure_dto
            .splines
            .clone()
            .into_iter()
            .map(|spline| spline.y1)
            .collect();

        let expected_x = vec![
            0.0, 0.01, 0.5, 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 20., 30., 40., 50., 60., 70.,
            80., 90., 100., 125., 150., 175., 200., 300., 400., 500., 1000., 2000., 3000., 4000.,
            5000., 6000., 7000., 8000., 9000., 10000., 15000.,
        ];
        let actual_x: Vec<f32> = structure_dto
            .splines
            .clone()
            .into_iter()
            .map(|spline| spline.x1)
            .collect();

        assert_eq!(expected_x, actual_x);
        assert_eq!(expected_y, actual_y);
    }
}
