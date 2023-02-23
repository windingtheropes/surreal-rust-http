# surreal http
surreal-http is a straight-to-the-point library to manage your SurrealDB database in rust, supporting raw SurrealQL commands, and receiving responses.
## getting started
Getting started with surreal-http is super simple:
``` 
    let address = String::from("localhost:8000");
    println!("Connecting to SurrealDB at {}", &address);

    let config = DbConfig {
        database: "db".to_string(),
        namespace: "ns".to_string(),
        user: "user".to_string(),
        pass: "pass".to_string(),
        address: "localhost:8000".clone(),
    };

    let mut handler = DbHandler::new(config).unwrap();

    handler.run_command("CREATE user SET name == \"Jack\"".to_string());
```
# surreal cmd
surreal-cmd is a simple command line application for querying and manipulating your SurrealDB database over http, with input and output functionality built in. 
## getting started
To get started with surreal-cmd, download the executable, and then run it in your terminal of choice, filling in arguments for the address, database, namespace, and credentials such like below:
```
surrealCmd.exe --address=localhost:8000 --ns=test --db=test --user=root --pass=root
```
Upon connection, you'll be able to enter any SurrealQL commands— and receive readable output —in the simple and fast Rust powered shell.\
When you're done, type EXIT to terminate the session. 
