use crate::smartcore::{smartcore_diseased, smartcore_healthy};
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn smallimage(
    path1: &str,
    path2: &str,
    widthinput: &str,
    heightinput: &str,
) -> Result<String, Box<dyn Error>> {
    let image_diseased = smartcore_diseased(path2, widthinput, heightinput).unwrap();
    let image_healthy = smartcore_healthy(path1, widthinput, heightinput).unwrap();

    let classifylabel_diseased = (0..image_diseased.len()).map(|_| 0).collect::<Vec<_>>();
    let classifylabel_healthy = (0..image_healthy.len()).map(|_| 0).collect::<Vec<_>>();

    let classiftvector_a = ndarray::arr1(&classifylabel_diseased);
    let classifyvector_b = ndarray::arr1(&classifylabel_healthy);
    let combinedvector = classiftvector_a + classifyvector_b;
    let finallabel = combinedvector.to_vec();

    let mut imagecombined: Vec<Vec<f32>> = Vec::new();
    for i in image_diseased.iter() {
        imagecombined.push(i.clone());
    }
    for i in image_healthy.iter() {
        imagecombined.push(i.clone());
    }

    let combinedtensor = DenseMatrix::from_2d_vec(&imagecombined).unwrap();

    let splitratio = train_test_split(&combinedtensor, &finallabel, 0.80, true, Some(28));
    let x_train = splitratio.0;
    let x_train_labels = splitratio.2;
    let y_train = splitratio.1;
    let y_labels = splitratio.3;

    let logistic_train =
        RandomForestClassifier::fit(&x_train, &x_train_labels, Default::default()).unwrap();
    let logistic_predict = logistic_train.predict(&y_train).unwrap();

    let accuracy_model = accuracy(&y_labels, &logistic_predict);

    println!(
        "The accuracy of the model using the Random forest: {}",
        accuracy_model,
    );

    Ok("Smartcore Random forest and SVM has completed".to_string())
}
