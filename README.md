# Candyman
CLI (Not TUI) http client that also supports GraphQL :)

### Example request
```yaml
[Uri]
http://localhost:9000/graphql

[Method]
POST

[Body::GraphQLQuery]
{
  apiVersion
}

[Body::GraphQLVariables]
{
  "id": 1
}
```
