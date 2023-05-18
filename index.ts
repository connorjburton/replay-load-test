// import { DB } from "https://deno.land/x/sqlite/mod.ts";

// // Open a database
// const db = new DB("database.db");
// db.execute(`
//   CREATE TABLE IF NOT EXISTS people (
//     id INTEGER PRIMARY KEY AUTOINCREMENT,
//     name TEXT
//   )
// `);

// // Run a simple query
// for (const name of ["Peter Parker", "Clark Kent", "Bruce Wayne"]) {
//   db.query("INSERT INTO people (name) VALUES (?)", [name]);
// }

// // Print out data in table
// for (const [name] of db.query("SELECT name FROM people")) {
//   console.log(name);
// }

// // Close connection
// db.close();

import testData from './test-data.json' assert { type: "json" };

const total = testData.length;
let success = 0;
let failure = 0;

performance.mark('start-requests');

for await (const data of testData) {
  const url = new URL(`http://bin-web-app:8080/${data.url.path}`);
  const resp = await fetch(url, {
    method: data.http.request.method
  });

  console.log(`Request to ${data.http.request.method} ${url} was successful with status code ${resp.status}`);

  if (resp.status === 200) {
    success++;
  } else {
    failure++;
  }
}

performance.mark('finish-requests');

console.log(`Finished making requests after ${performance.measure('requests', 'start-requests', 'finish-requests').duration}ms, ${success}/${total} succeeded, ${failure} failed`);
