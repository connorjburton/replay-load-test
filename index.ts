import testData from './test-data.json' assert { type: "json" };

const timestampToNumber = (data) => {
  return {
    ...data,
    timestamp: {
      us: Number(data.timestamp.us)
    }
  }
}

const calculateDeltas = (data) => data.map(({ timestamp }) => timestamp.us - data[0].timestamp.us);

const wrapFinish = async (fn) => {
  await fn();
  performance.mark('finish-requests');

  console.log(`Finished making requests after ${performance.measure('requests', 'start-requests', 'finish-requests').duration}ms, ${success}/${total} succeeded, ${failure} failed`);
}

const sendRequest = async (data, scheduled) => {
  const now = new Date().getTime();
  console.log(`Called sendRequest, was scheduled at ${scheduled}, is now ${now}, took ${now - scheduled}ms between scheduling and triggering`);
  const url = new URL(`http://bin-web-app:8080/${data.url.path}`);
  console.log(`Sending request to ${data.http.request.method} ${url} at ${new Date()}`);
  const resp = await fetch(url, {
    method: data.http.request.method,
    headers: data.http.request.headers
  });

  console.log(`Request to ${data.http.request.method} ${url} was successful with status code ${resp.status}`);

  if (resp.status === 200) {
    success++;
  } else {
    failure++;
  }
};

// STEP 1:
// - Take data from any source
// - Convert it into a structure we expect
// - Structure should normalize the timestamp to be a delta

const sortedData = testData.map(timestampToNumber).sort((a, b) => a.timestamp.us - b.timestamp.us);
const deltas = calculateDeltas(sortedData);
console.log('Length of deltas is ', deltas.length);
console.log('Length of data is ', sortedData.length);

// STEP 2:
// - Set timeouts for every timestamp
// - Each timeout will perform all the requests for that millisecond
// - As we start at 0 the timers will start executing as we start setting them
// - TODO: Batch these so we don't set thousands of timers
const total = sortedData.length;
let success = 0;
let failure = 0;

performance.mark('start-requests');

console.log('Starting requests');

const timers: number[] = [];

deltas.forEach((delta, idx) => {
  const send = sendRequest.bind(null, sortedData[idx], new Date().getTime());
  const callback = idx === deltas.length - 1 ? wrapFinish.bind(null, send) : send;
  timers.push(setTimeout(callback, delta));
});

console.log(`Set all timers, currently have ${timers.length}`);
