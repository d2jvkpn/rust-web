import { post } from "./base.js";

export function sendMsg(msg, callback) {
  post("/api/auth/user/chatgpt/chat/completions", msg, callback)
}
