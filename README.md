# surreal http
surreal-http is a straight-to-the-point library to manage your SurrealDB database in rust, supporting raw SurrealQL commands, and receiving responses.
## getting started
Getting started with surreal-http is super simple:
``` 
    let address = "localhost:8000".to_string()
    println!("Connecting to SurrealDB at {}", &address);

    let config = DbConfig {
        database: "db".to_string(),
        namespace: "ns".to_string(),
        user: "user".to_string(),
        pass: "pass".to_string(),
        address: "localhost:8000".clone(),
    };

    let mut handler = DbHandler::new(config);

    handler.run_command("CREATE user SET name == \"Jack\"");
```
# surreal cmd
surreal-cmd is a simple command line application for querying and manipulating your SurrealDB database over http, with input and output functionality built in. 
## getting started
To get started with surreal-cmd, download the executable, and then run it in your terminal of choice.