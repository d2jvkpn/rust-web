import { redirectTo, post } from "./base.js";

export function login(data) {
  post("/api/open/user/login", data, function(res) {
    localStorage.setItem("tokens", JSON.stringify(res.data.tokens));
    localStorage.setItem("user", JSON.stringify(res.data.user));
    redirectTo("/home");
  })
}

export function getUser() {
   let str = localStorage.getItem("user");
   if (!str) {
      return null;
   }

   return JSON.parse(str);
}
