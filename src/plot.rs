use std::path::Path;
use plotters::prelude::*;

pub fn plot_membrane_potential(
    fig_path: &Path, v: &Vec<f64>, t: &Vec<f64>
) -> Result<(), Box<dyn std::error::Error>> {
    let v_min = v.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let v_max = v.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let t_min = t.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let t_max = t.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let root = BitMapBackend::new(fig_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Membrane Potential", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d((t_min-1.)..(t_max+1.), (v_min-1.)..(v_max+1.))?;

    chart.configure_mesh().x_desc("time (ms)").y_desc("voltage (mV)").draw()?;

    chart
        .draw_series(LineSeries::new(
            t.iter().zip(v.iter()).map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("membrane potential")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn plot_phase_plane_trajectory(
    fig_path: &Path, x1: &Vec<f64>, x2: &Vec<f64>, x1_name: &str, x2_name: &str
) -> Result<(), Box<dyn std::error::Error>>{
    let x1_min = x1.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let x1_max = x1.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let x2_min = x2.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let x2_max = x2.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let root = BitMapBackend::new(fig_path, (720, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Phase Plane", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d((x1_min-1.)..(x1_max+1.), (x2_min-1.)..(x2_max+1.))?;

    chart.configure_mesh()
        .x_desc(x1_name)
        .y_desc(x2_name)
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            x1.iter().zip(x2.iter()).map(|(x, y)| (*x, *y)),
            &RED,
        ))?
        .label("trajectory")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_membrane_potential() {
        let fig_path = Path::new("test.png");
        let v = vec![-1., 4., -9., 16., -25.];
        let t = vec![1., 2., 3., 4., 5.];
        plot_membrane_potential(fig_path, &v, &t).unwrap();
    }

    #[test]
    fn test_plot_phase_plane_trajectory() {
        let fig_path = Path::new("test.png");
        let x1 = vec![-1., 4., -9., 16., -25., -16., 9., -4., 1.];
        let x2 = vec![1., 2., 3., 4., 5., 4., 3., 2., 1.];
        plot_phase_plane_trajectory(fig_path, &x1, &x2, "x1", "x2").unwrap();
    }
}