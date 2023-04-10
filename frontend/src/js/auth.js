import { redirectTo, post } from "./base.js";

var RresheToken = null;
const Interval = 10*1000;

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

function refreshToken() {
  var str = localStorage.getItem("tokens");
  if (!str) {
    return;
  }

  let tokens = JSON.parse(str);
  let ts = new Date().getTime();

  let delta = tokens.refreshExp*1000 - ts;
  if (delta < Interval) {
    clearInterval(RresheToken);
    redirectTo("/login");
    return;
  }

  delta = tokens.accessExp*1000 - ts;
  if (delta > Interval) {
    // console.log(`~~~ no need to refreshToken`);
    return;
  }

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
