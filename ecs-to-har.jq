{
    log: {
        version: "1.2",
        creator: {
            name: "Your Name",
            version: "1.0"
        },
        entries: [
            .[] | {
                startedDateTime: .["@timestamp"],
                time: 0,
                request: {
                    method: .http.request.method,
                    url: .request,
                    httpVersion: "HTTP/1.1",
                    cookies: [],
                    headers: .http.request.headers | to_entries | map(.name = .key | del(.key)),
                    queryString: [],
                    headersSize: 0,
                    bodySize: 0
                },
                response: {
                    status: .status,
                    statusText: "OK",
                    httpVersion: "HTTP/1.1",
                    cookies: [],
                    headers: [],
                    content: {
                        size: .bytes,
                        mimeType: "application/octet-stream"
                    },
                    redirectURL: "",
                    headersSize: 0,
                    bodySize: 0
                },
                cache: {},
                timings: {
                    blocked: 0,
                    dns: -1,
                    connect: -1,
                    send: 0,
                    wait: 0,
                    receive: 0,
                    ssl: -1
                },
                serverIPAddress: .host,
                connection: "",
                comment: ""
            }
        ]
    }
}
