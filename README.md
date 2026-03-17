# four0three

four0three is a 401/403 bypass fuzzer built based on BypassFuzzer.py, but written in rust.
credits: intrudier https://github.com/intrudir/

It goes brrrr.

### Usage:

```
Options:
  -r, --request <REQUEST>                  Path to request file.
      --scheme-override <SCHEME_OVERRIDE>  URL scheme to use, 'http', 'https'.
  -u, --url <URL>                          URL to fuzz.
  -X, --method <METHOD>                    Method to use with the request URL.
  -d, --data <DATA>                        Post body data for the request.
  -c, --cookies <COOKIES>                  Extra Cookies to add to the request.
  -H, --headers <HEADERS>                  Extra header and value to add to the request.
  -h, --help                               Print help

Proxy Settings:
      --burp <BURP>  Burp proxy address. (Format: http://host:port)

Performance Settings:
  -t, --threads <THREADS>        Number of threads to use. This is the amount of workers that are able to modify and send requests at a time. [default: 50]
      --queue-size <QUEUE_SIZE>  Limit the max number of requests waiting to be processed by a thread. Higher numbers increase memory consumption for better performance. [ Low: 5, High: 50 ] [default: 100]
  -R, --rate-limit <RATE_LIMIT>  Max number of requests that can be sent at once. [default: 20]

Client Settings:
  -k, --insecure      Skip SSL/TLS certificate validation.
  -A, --random-agent  Replace the Agent header value with a random agent.

Response Settings:
  -f, --follow-redirects  Follow redirects.

Payload Settings:
      --skip-ip-payloads
          Skip all IP payloads.
      --skip-url-payloads
          Skip all URL payloads.
  -D, --oob-domain-payload <OOB_DOMAIN_PAYLOAD>
          Out-of-band domain. Adds more tests. Can be the same value as -P. (Format: -D '192.168.0.1' or -D 'some.domain.com')
  -P, --oob-payload <OOB_PAYLOAD>
          Out-of-band payload. Adds more tests. Can be the same value as -D. (Format: -P '192.168.0.1' or -P 'some.domain.com')
  -b, --base64
          Try header payload values as is and with base64 encoding header values. Because try harder.
      --url-encode
          Try header payload values as is and as URL encoded payloads. Because try harder. This will URL encode payloads that are already URL encoded.
      --case-tamper
          Try header payload values as is and with upper case. Because try harder.
      --extra-header-payloads <EXTRA_HEADER_PAYLOADS>
          Extra header payloads to be used in the payload template. Can be a string list seperated by commas. For a list of header payloads, use --list-header-payloads. (Format: 'Header: Value' or 'Header: Value, Header: Value,...')
      --extra-ip-payloads <EXTRA_IP_PAYLOADS>
          Extra IP payloads to be used in the ip payload template. Can be a string list seperated by commas. For a list of IP payloads, use --list-ip-payloads. (Format: '192.168.0.1' or '192.168.0.1, 127.0.0.1,...')
      --extra-url-payloads <EXTRA_URL_PAYLOADS>
          Extra URL payloads to be used in the URL payload template. Can be a string list seperated by commas. For a list of URL payloads, use --list-url-payloads. (Format ';/../' or ';/../, ;/../.;/../,...')

General Settings:
  -v, --verbose                  Enable verbose output.
      --log-output <LOG_OUTPUT>  Log any output to file.
```
