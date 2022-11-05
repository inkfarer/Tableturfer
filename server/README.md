# tableturfer-server

The backend component of Tableturfer.

## Configuration

### Using a config file

Copy `cfg/config_example.toml` into a new file at `cfg/config.toml`. The application can be configured from this file.

### Using environment variables

The application can also be configured through environment variables.   
For example, the following two blocks contain the same configuration; one using environment variables, the other using a
TOML file.

```
TBLT_REDIS.HOST=localhost
TBLT_REDIS.PORT=3948
```

```toml
[redis]
host = "localhost"
port = 3948
```
