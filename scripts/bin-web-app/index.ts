const server = Deno.listen({ port: 8080 });
console.log('HTTP bin webserver running. Access it as: http://localhost:8080/');

for await (const conn of server) {
	serveHttp(conn);
}

async function serveHttp(conn: Deno.Conn) {
	const httpConn = Deno.serveHttp(conn);

	for await (const requestEvent of httpConn) {
		const body = `Your user-agent is:\n\n${
			requestEvent.request.headers.get("user-agent") ?? "Unknown"
		}`;
		
		console.log(`Received request to ${requestEvent.request.method} ${requestEvent.request.url} with user agent ${requestEvent.request.headers.get('User-Agent')}`);

    	requestEvent.respondWith(
			new Response(body, {
				status: 200,
			}),
      	);
    }
}