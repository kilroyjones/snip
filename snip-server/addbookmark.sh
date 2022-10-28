curl -v -X POST http://localhost:8080/add-bookmark -H 'Content-Type: application/json' -d @- <<BODY
{
    "link": "any textual information, like the contents of a page or quote", 
    "source": "location from where it came",
    "description": "optional description",
    "tags": []
}
BODY
