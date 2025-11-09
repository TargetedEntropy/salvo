use salvo_backend::services::material_calculator;

#[test]
fn test_reprocessing_efficiency_no_skills() {
    let efficiency = material_calculator::calculate_reprocessing_efficiency(0, 0, 0);
    assert_eq!(efficiency, 0.5, "Base efficiency should be 50%");
}

#[test]
fn test_reprocessing_efficiency_max_skills() {
    let efficiency = material_calculator::calculate_reprocessing_efficiency(5, 5, 5);
    // 0.5 * 1.15 * 1.10 * 1.10 = 0.69575
    assert!(
        (efficiency - 0.69575).abs() < 0.0001,
        "Max skills should give ~69.575% efficiency, got {}",
        efficiency
    );
}

#[test]
fn test_reprocessing_efficiency_partial_skills() {
    let efficiency = material_calculator::calculate_reprocessing_efficiency(5, 4, 3);
    // 0.5 * 1.15 * 1.08 * 1.06 = 0.65826
    assert!(
        (efficiency - 0.65826).abs() < 0.001,
        "Partial skills should give ~65.83% efficiency, got {}",
        efficiency
    );
}
