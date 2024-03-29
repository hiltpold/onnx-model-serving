# ONNX Model Serving

## Start
* `clone the repo`
* `cargo run`

##  Test
* run `./send_test_image.sh`. This sends the `./test/test.jpg` folder and stores the result under `./test/pred/test.jpg`.
* if `jq` is not available execute `install sudo apt-get install jq`

## Model
* train a YOLOv7 model following the description in the original [repo](https://github.com/WongKinYiu/yolov7)
* export the trained model to ONNX with NMS (Non-Max-Suppression) for inference
* put the the model in the folder `./model/`
* currently a YOLOv7 model trained on the [militaryaircrafts](https://www.kaggle.com/datasets/a2015003713/militaryaircraftdetectiondataset) is provided

## Docker
* export PROJECT="onnx-model-serving"
* build image `docker build -t onnx-model-serving:latest --build-arg project=$PROJECT -f ./docker/dockerfile .`
* launch container `docker run -p 5000:5000 onnx-model-serving:latest`
* test as previously explained

## Endpoints

| endpoint  | method  | payload  | response  |   |
|---|---|---|---|---|
|  api/v1/heartbeat  |  GET | -  |   |  {"status": "running"} |
|  /api/v1/objectdetection | POST  | {"image_format": "jpg", "model_name":"militaryaircrafts.onnx", "image": <base64_encoded_image>"} |  {"result_image": <base64_encoded_image_with_bounding_boxes>, "bounding_boxes": [{"min_x": x, "min_y":y,"height": h, "width": w, "class": "c", "conf": c }, ...]} |   |

# Todo
* better error handling

