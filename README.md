# HTTT

A minimal malicious HTTP server that responds over a uniform interval. 

This is bad for an HTTP client that considers a request to be timed out if the underlying
TCP read times out. For example, Python's `requests` module.

## Installation

```sh
cargo install --git https://github.com/aalekhpatel07/httt.git
```

## Usage

- Start an HTTP server on port `9000` that transmits the bytes `HTTP/1.1 200 OK\r\n\r\n`  one byte at a time alternating between writing and sleeping for `1` second.

```sh
httt --interval 1 --port 9000
```

## Testing

- First, start the server on port 9000 that sends one byte every `200`ms (i.e. it takes ~`3.8`s to send all of `19` bytes):
```sh
httt --interval 0.2 --port 9000
```

- Then, open a Python shell that has `requests` installed. And observe that the `timeout` kwarg in `requests.get` does not imply a request timeout:
```python
>>> import requests
>>> import time
>>>
>>> def time_it():
...     start = time.perf_counter_ns()
...     response = requests.get("http://localhost:9000", timeout=1)
...
...     # Notice how even though `timeout=1` suggests the request 
...     # will timeout if it takes longer than 1 second, but it actually takes 
...     # around 3 seconds before completing and it is allowed to run uninterrupted.
...
...     end = time.perf_counter_ns()
...     print("request took: {(end - start)/1e9:.3f} seconds")
...
>>> time_it()
request took: 3.823 seconds
```

- If on the other hand you `curl` the server with a `3`s timeout, then it does close the connection prematurely, as expected:
```
$ curl -v -m 3 http://localhost:9000

*   Trying 127.0.0.1:9000...
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET / HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/8.0.1
> Accept: */*
>
* Operation timed out after 3000 milliseconds with 0 bytes received
* Closing connection 0
curl: (28) Operation timed out after 3000 milliseconds with 0 bytes received
```
