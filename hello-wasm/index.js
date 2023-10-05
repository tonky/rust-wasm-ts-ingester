import { greet, message, MetricV1, V1  } from "./pkg/index.js";

const button_v1 = document.getElementById("send_v1");
button_v1.addEventListener("click", (event) => {
  const id_text = Number(document.getElementById("id").value);
  const id = Number(id_text);
  const token = document.getElementById("token").value;
  const dt_text = document.getElementById("datetime").value;
  const dt = Number(dt_text);

  console.log(`Parsed: id: token: dt`, id_text, id, token, dt_text, dt);

  var msg = new MetricV1(id, token, dt).msgpk_b64();
  // const m = new MetricV1(1, "token3", 12345);

  console.log("posting metricv1: {}", msg);

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
    const response = await fetch("http://localhost:3000/v1/ingest", {
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

const v1 = new V1();
const m = new MetricV1(1, "token3", 12345);

console.log(">> js v1 | publish : {}", v1.publish(m));

