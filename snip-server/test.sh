curl -v -X POST http://localhost:8080/post-snippet -H 'Content-Type: application/json' -d @- <<BODY
{
    "content": "any textual information, like the contents of a page or quote", 
    "source": "location from where it came",
    "description": "optional description" 
}
BODY
