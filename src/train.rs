use crate::bleached::makevecdiseasedcummulative;
use crate::healthy::makevechealthycummulative;
use axonml::data::transforms::Flatten;
use axonml::nn::Conv2d;
use axonml::nn::CrossEntropyLoss;
use axonml::nn::Linear;
use axonml::nn::MaxPool2d;
use axonml::nn::Module;
use axonml::nn::ReLU;
use axonml::nn::Sequential;
use axonml::optim::Adam;
use axonml::optim::Optimizer;
use axonml::prelude::Variable;
use axonml::tensor::Tensor;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn conv2d(healthy: &str, leached: &str) -> Result<String, Box<dyn Error>> {
    let model = Sequential::new()
        .add(Conv2d::new(1, 32, 3))
        .add(ReLU)
        .add(MaxPool2d::new(2))
        .add(Conv2d::new(32, 64, 3))
        .add(ReLU)
        .add(MaxPool2d::new(2))
        .add(Flatten)
        .add(Linear::new(16, 1));

    let mut optimizer = Adam::new(model.parameters(), 0.001);

    let datasethealthy = makevechealthycummulative(healthy).unwrap();
    let diseased = makevecdiseasedcummulative(leached).unwrap();

    let mut combined: Vec<f32> = Vec::new();
    let mut combineddata: Vec<Vec<f32>> = Vec::new();

    for i in 0..datasethealthy.0.len() {
        combineddata.push(datasethealthy.0[i]);
        combineddata.push(diseased.0[i]);
        combined.push(datasethealthy.1[i]);
        combined.push(diseased.1[i]);
    }

    let finaldata = vec![(combineddata, combined)];

    for epoc in 0..100 {
        let mut total_loss = 0.0f32;
        for (i, val) in &finaldata {
            let image =
                Tensor::from_vec(i.iter().flatten().cloned().collect::<Vec<_>>(), &[0]).unwrap();
            let input = Variable::new(image, false);
            let targetval = Variable::new(Tensor::from_slice(val.as_slice(), &[0]).unwrap(), false);
            optimizer.zero_grad();
            let output = model.forward(&input);
            let lossvalue = CrossEntropyLoss::new().compute(&output, &targetval);
            optimizer.zero_grad();
            lossvalue.backward();
            optimizer.step();
            println!("Epoch: {:?}, loss: {:?}", epoc, lossvalue);
        }
    }
    Ok("The model has finished".to_string())
}
