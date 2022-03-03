# log-converter

The log converter converts text logs to a json that GKE will be happy with. It tries to detect the log level. If no log level can be found, it will default to INFO.

## Usage

It reads from stdin and writes to stdout, so it can be used in a pipe:

```
programme | log-converter
```

For example

```
$ echo 'WARN this is not good!' | ./target/release/log-converter 
{"severity":"WARN","message":"WARN this is not good!","timestamp":{"seconds":1646331031,"nanos":957710634}}
```

You might want to redirect stderr as well. 

## Build

cargo build

## Test

cargo test

## Deployment

Since the binary is statically linked, it can easily be added to any docker image just by copying the binary. It works on any linux distribution, including Alpine. See the example Dockerfile in this repo.



