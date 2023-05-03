# ONNX YOLOv7 Model Serving

# Start
* `clone the repo`
* `cargo run`

#  Test
* run `./send_test_image.sh`. This sends the `./data/test/test.jpg` folder and stores the result under `data/pred/test.jpg`.
* if `jq` is not available execute `install sudo apt-get install jq`

# Model
* train a YOLOv7 model following the description in the original [repo](https://github.com/WongKinYiu/yolov7)
* export the trained model to ONNX with NMS (Non-Max-Suppression) for inference
* put the the model in the folder `./model/`
* currently a YOLOv7 model trained on the [militaryaircrafts](https://www.kaggle.com/datasets/a2015003713/militaryaircraftdetectiondataset) is provided

# Docker
* export PROJECT="onnx-model-serving"
* Build image `docker build -t onnx-model-serving:latest --build-arg project=$PROJECT .`
* Launch container `docker run -p 5000:5000 onnx-model-serving:latest`

# Todo
* better error handling