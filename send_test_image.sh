# check heartbeat 
STATUS=$(curl http://localhost:8080/api/v1/heartbeat | jq '.status')
OK='"running"'
if [[ $STATUS != $OK ]]
then 
	echo "Server is not running"
	exit 1
fi
# detect military aircrafts in the base64 encoded image
RESULT=$((echo -n '{"image_name": "test", "image_format": "jpg", "model_name":"militaryaircrafts.onnx", "image": "'; base64 ./data/test/test.jpg; echo '"}') | curl -H "Content-Type: application/json" -d @-  http://localhost:8080/api/v1/objectdetection)
RESULT_IMAGE=$(echo $RESULT | jq '.result_image')
echo $RESULT_IMAGE | base64 -d --ignore-garbage > ./data/pred/test.jpg
