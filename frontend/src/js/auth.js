import { redirectTo, post } from "./base.js";
import { datetime } from "./utils.js";

var RresheToken = null;
const Interval = 15*1000; // 15s

export function login(data) {
  post("/api/open/user/login", data, function(res) {
    localStorage.setItem("tokens", JSON.stringify(res.data.tokens));
    localStorage.setItem("user", JSON.stringify(res.data.user));
    redirectTo("/home");
  });
}

export function setRefreshToken() {
  if (RresheToken) {
    return;
  }

  RresheToken = setInterval(refreshToken, Interval);
}

export function refreshToken() {
  var str = localStorage.getItem("tokens");
  if (!str) {
    return;
  }

  let tokens = JSON.parse(str);
  let now = datetime();
  let mts = now.getTime();

  let delta = tokens.refreshExp*1000 - mts;
  if (delta < Interval) {
    clearInterval(RresheToken);
    redirectTo("/login");
    return;
  }

  delta = tokens.accessExp*1000 - mts;
  if (delta > Interval) {
    // console.log(`~~~ ${now.rfc3339} no need to call refreshToken`);
    return;
  }
  // console.log(`--> ${now.rfc3339} call refreshToken`);

  let data = {refreshToken: tokens.refreshToken};

  post("/api/open/user/refresh_token", data, function(res) {
    localStorage.setItem("tokens", JSON.stringify(res.data));
  });
}

export function getUser() {
   let str = localStorage.getItem("user");
   if (!str) {
      return null;
   }

   return JSON.parse(str);
}
