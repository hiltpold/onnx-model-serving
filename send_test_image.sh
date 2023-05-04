# check heartbeat 
STATUS=$(curl http://localhost:5000/api/v1/heartbeat | jq '.status')
OK='"running"'
if [[ $STATUS != $OK ]]
then 
	echo "Server is not running"
	exit 1
fi
# detect military aircrafts in the base64 encoded image
RESULT=$((echo -n '{"image_format": "jpg", "model_name":"militaryaircrafts.onnx", "image": "'; base64 ./test/test.jpg; echo '"}') | curl -H "Content-Type: application/json" -d @-  http://localhost:5000/api/v1/objectdetection)
RESULT_IMAGE=$(echo $RESULT | jq '.result_image')
echo $RESULT_IMAGE | base64 -d --ignore-garbage > ./test/pred/test.jpg
