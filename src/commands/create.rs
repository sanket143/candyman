use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn md_file_template() -> &'static str {
    r#"
# Request
```toml
uri = "https://api.mocki.io/v2/c4d7a195/graphql"
method = "POST"
```
# Headers
```json
{
  "authorization": "token"
}
```

# Body
```graphql
# This could be json or text body depending on the requirement
# For GraphQL query, define the next block as json block to identify them as variables
query {
  users {
    id
    name
  }
}
```
```json
{
  "id": 1
}
```

# [Test] Test 1
```js
// the response of the request will be available in `RESPONSE` variable
print(RESPONSE)
```

# [Test] Test 2
```js
print("TEST:", RESPONSE["data"])
```
"#
}

pub fn call(filepath: &str) -> Result<()> {
    let filepath = &filepath;
    let filepath = Path::new(filepath);
    let folder = filepath.parent().unwrap_or(Path::new("./"));

    if !folder.exists() {
        fs::create_dir_all(&folder).expect("Failed to create directory for the file");
    }

    let mut file = File::create(filepath)?;
    file.write_all(md_file_template().as_bytes())?;

    Ok(())
}
