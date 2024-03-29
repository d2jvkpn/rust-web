import { post, get } from "./base.js";
import { datetime } from "./utils.js";

export function sendMsg(msg, callback) {
  post("/api/auth/user/chat/completions", msg, callback)
}

export function chatQuery(callback) {
  get("/api/auth/user/chat/query", {page_no: 1, page_size: 20}, callback);
}

export function chatItems2Msgs(items) {
  let messages = []

  items.forEach((e) => {
    let at, msg;

    if (e.response) {
      at = datetime(e.responseAt);

      msg = {
        sender: "system",
        role: "assistant",
        content: e.response,
        timestampMilli: at.getTime(),
        at: at.time,
      };
      messages.push(msg);
    }

    at = datetime(e.queryAt);

    msg = {
      sender: "user",
      role: "user",
      content: e.query,
      timestampMilli: at.getTime(),
      at: at.time,
    };

    messages.push(msg);
  });

  return messages;
}
