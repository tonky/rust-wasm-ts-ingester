import { greet, message, NaiveDateTime, post_metric } from "./pkg";
// import { new_v1 } from "./pkg";
import { V2, Metric, DateTime } from "./pkg";

const button_v2 = document.getElementById("send_v2");

button_v2?.addEventListener("click", (event) => {
  // const id_text = <HTMLInputElement>document.getElementById("id2");
  const id_text = document.getElementById("id2")?.value
  const id = Number(id_text);

  var token = document.getElementById("token2")?.value;

  const dt_year = Number(document.getElementById("dt_year")?.value);
  const dt_ms = Number(document.getElementById("dt_ms")?.value);

  const v2 = new V2();
  const dt = new DateTime(dt_year, dt_ms);
  const m = new Metric(1, token, dt);
  const res_v2 = v2.publish(m);
  res_v2
    .then((resp) => {
      console.log("resp v2: ", resp);
      document.getElementById("result-post-v2").innerText = JSON.stringify(resp);
    })
    .catch((err) => {console.log("caught error: ", err)});
});