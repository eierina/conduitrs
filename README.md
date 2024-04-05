Just a PoC work in progress.

[API Test Collection](api_test.postman_collection.json)

curl -X 'POST' \
```shell
  'http://localhost:8080/proposals' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "proposalId": "4fa3e13d-3e6c-4226-a8ac-14161efb28a8",
  "participants": [
    "7ae556d5-307b-426f-a815-c4377067a51e",
    "57a98d98-c34e-44db-83f7-065e24a81481",
    "6bfaa980-2340-4c79-b902-2b5c6f32a81c"
  ],
  "payloadHint": "bond-issuance",
  "payload": {
    "finalOffering": {
      "asset": "b019e6b6-e477-4766-b39d-ed04c4c7d916",
      "network": "OilTrading",
      "dataDictionary": {
        "Type": "Net.Trading.OilState",
        "Grade": "Sweet Crude Oil",
        "Barrels": "50",
        "Producer": "Big Oil Refinery Ltd"
      }
    },
    "finalBid": {
      "asset": "2284f6b3-8473-476c-b39a-edab84c2d010",
      "network": "DigitalUSD",
      "dataDictionary": {
        "TokenDef": "Net.Currency.USD",
        "Value": "5000"
      }
    }
  }
}'
```

```shell
curl -X 'POST' \
  'http://localhost:8080/proposals' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "id": "4fa3e13d-3e6c-4226-a8ac-14161efb28a8",
  "maker_id": "7ae556d5-307b-426f-a815-c4377067a51e",
  "payload_hint": "bond-issuance",
  "payload": "hey there!"
  }'
```