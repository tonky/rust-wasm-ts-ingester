import { greet, message } from "./pkg/index.js";

const button_v1 = document.getElementById("send_v1");
button_v1.addEventListener("click", (event) => {
  var msg = message(2, "token2", "2014-12-28T12:00:09Z");

  console.log("posting with token2");

  postJSON(msg);
});

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

