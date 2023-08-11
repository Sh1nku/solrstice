use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use solrstice::models::error::SolrError;
use solrstice::queries::components::facet::{FacetSetComponentBuilder, PivotFacetComponentBuilder};
use solrstice::queries::index::UpdateQueryBuilder;
use solrstice::queries::select::SelectQueryBuilder;

#[tokio::test]
pub async fn test_facet_pivot_works() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("FacetPivot")
        .await
        .unwrap();
    let update = UpdateQueryBuilder::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQueryBuilder::new().facets(
        &FacetSetComponentBuilder::new()
            .pivots(&PivotFacetComponentBuilder::new(&["interests,age"])),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facets().ok_or("No facets")?;
    let pivot = facets.get_pivots().ok_or("No pivot")?;
    let interests_age = pivot.get("interests,age").ok_or("No interests,age pivot")?;
    assert_eq!(interests_age.len(), 3);
    let cars_pivot = interests_age
        .iter()
        .find(|p| match p.get_value::<String>() {
            Ok(val) => val.as_str() == "cars",
            Err(_) => false,
        })
        .ok_or("No cars pivot")?;
    assert_eq!(cars_pivot.count, 1);
    let age_pivot = cars_pivot
        .get_pivots()
        .ok_or("No pivots for cars")?
        .first()
        .ok_or("No age pivot in cars")?;
    assert_eq!(age_pivot.get_value::<usize>()?, 20);

    let _ = config.tear_down().await;
    Ok(())
}
