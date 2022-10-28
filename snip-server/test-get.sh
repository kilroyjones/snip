curl -v -X POST http://localhost:8080/get-quotes -H 'Content-Type: application/json' -d @- <<BODY
{
    "limit": 10, 
    "offset": 1,
    "trash": false
}
BODY
