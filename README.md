# smsapi

French mobile operator "Free Mobile" provides as part of one's subscription options an API dubbed "SMS API" that allows to programmatically send SMS to yourself by making requests to an HTTP endpoint.
This little project is :
 - a Rust library to interface with said API
 - a small CLI executable to send SMSs from the command line.
 
# Usage

```
smsapi 0.1.0

USAGE:
    smsapi --user <USER> --api-key <API_KEY> [MESSAGE]

ARGS:
    <MESSAGE>    

OPTIONS:
    -h, --help                 Print help information
    -k, --api-key <API_KEY>    
    -u, --user <USER>          
    -V, --version              Print version information
```
