// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
/*
const rust = import('./pkg');

console.log("running2...");

rust
  .then(m => console.log(m.greet('World!')))
  .catch(console.error);


rust
  .then(m => console.log(m.message(1, "token", "2014-12-28T12:00:09Z")))
  .catch(console.error);
*/

// import * as wasm from "./pkg";
// const greeting  = wasm.greet("Joe");
import { greet, message } from "./pkg";
const greeting  = greet("Joe");
console.log("Joe");


const b64mp = message(1, "token", "2014-12-28T12:00:09Z");

document.getElementById("result-greet").innerText = greeting;
document.getElementById("result-message").innerText = b64mp;

const response = await fetch("http://localhost:3000/");
const hello = await response.text();

document.getElementById("result-fetch").innerText = hello;


async function postJSON(data) {
  try {
    const response = await fetch("http://localhost:3000/ingest", {
      method: "POST", // or 'PUT'
      headers: { "Content-Type": "application/json", },
      body: data,
    });

    const result = await response.json();

    document.getElementById("result-post").innerText = JSON.stringify(result);

    console.log("Success:", result);
  } catch (error) {
    console.error("Error:", error);
  }
}

postJSON(b64mp);

