import { message } from "antd";
import { refreshToken } from "./auth.js";

const Settings = {
  publicUrl: process.env.PUBLIC_URL || "",
  headers: {},
  apiAddress: "",
};

const Interval = 15*1000; // 15s

export function load(callback) {
  if (Settings.apiAddress !== "") {
    if (callback) {
      callback();
    }
    return;
  }

  let url = new URL(window.location.href);
  let jsonUrl = `${url.protocol}//${url.host}${Settings.publicUrl}/frontend.json`

  request(jsonUrl, {method: "GET", headers: {}}, function(d) {
    Settings.apiAddress = d.apiAddress;
    console.log(`==> Got configuration frontend.json: ${JSON.stringify(d)}`);
    setHeader("X-TZ-Offset", new Date().getTimezoneOffset());

    if (callback) {
      callback();
    }
  });
}

export function getPublicUrl() {
  return Settings.publicUrl;
}

export function redirectTo(p) {
  if (!p) {
    return;
  }
  window.location.href = `${Settings.publicUrl}${p}`;
}

function getTokens() {
   let str = localStorage.getItem("tokens");
   if (!str) {
      return null;
   }

   return JSON.parse(str);
}

export function authed() {
  let tokens = getTokens(); // authentication
  // console.log(`~~~ authed tokens: ${JSON.stringify(tokens)}`);
  if (!tokens || !tokens.accessToken) {
    return false;
  }

  let ts = Math.round(new Date().getTime());
  let delta = tokens.refreshExp*1000 - ts;

  if (delta < Interval) {
    return false;
  }

  if (tokens.accessExp*1000 <= ts) {
    refreshToken();
  }

  return true;
}

export function setHeader(key, value) {
  if (key) {
    Settings.headers[key] = value;
  }
}

export function post(path, data=null, callback=null) {
  let options = { method: "POST", headers: {...Settings.headers} };
  options.headers["Content-Type"] = "application/json";

  if (data) {
    options.body = JSON.stringify(data);
  }

  request(`${Settings.apiAddress}${path}`, options, callback);
}

export function get(path, parameters=null, callback=null) {
  let options = {method: "GET", headers: {...Settings.headers} };

  if (parameters) {
    let arr = [];

    for (const [key, value] of Object.entries(parameters)) {
      if (value) arr.push(`${key}=${value}`);
    }

    if (arr.length > 0) {
      path += `?${arr.join("&")}`;
    }
  }

  request(`${Settings.apiAddress}${path}`, options, callback);
}

export function request(path, options, callback=null) {
  let tokens = getTokens(); // authentication

  if (tokens && tokens.accessToken) {
    let now = new Date();
    if (tokens.refreshExp*1000 <= Math.round(now.getTime())) {
      redirectTo("/login");
      return;
    }

    options.headers["Authorization"] = `Bearer ${tokens.accessToken}`;
  }

  fetch(`${path}`, options)
    .then(response => {
      // let contentType = response.headers.get("Content-Type");
      // console.log(`~~~ got response: ${response.status}, ${response.length}, ${contentType}`);

      /* TODO: backend
      if (!contentType || !contentType.startsWith("application/json")) {
        throw new TypeError("invalid response");
      }
      */

      /*
      if (response.action) {
        switch (response.action) {
          case "login":
            // localStorage.getItem(name);
            // localStorage.setItem(name, value);
            localStorage.clear();
            window.location.href = `${Settings.publicUrl}/login`;
            return;
          default:
        }
      }
      */

      return response.json();
    }).then(res => {
      if (!res) {
        throw new TypeError("empty response");
      }

      // redirect to login
      if (res.code && res.code === 16) {
        localStorage.clear();
        redirectTo("/login");
        return;
      }

      if (res.code && res.code !== 0) {
        message.warning(res.msg);
        console.log(`!!! response error: code=${res.code}, msg=${res.msg}`);
        return;
      }

      if (res.code === 0 && options.method === "GET" && res.data.hasOwnProperty("items")) {
        if (Array.isArray(res.data.items) && res.data.items.length === 0) {
          message.warning("!!! Have no items");
          console.warn("!!! Have no items");
        }
      }

      if (callback) {
        callback(res);
      }
    })
    .catch(function (err) {
      console.error(`!!! http ${options.method} ${path}: ${err}`);
      handleFetchErr(err);
    });
}

function handleFetchErr(err) {
  let msg;

  if (err instanceof TypeError && err.message.startsWith("NetworkError")) {
    msg = "NetworkError: request failed";
  } else if (err instanceof TypeError)  {
    msg = `TypeError: ${err.message}`;
  } else if (err instanceof SyntaxError)  {
    msg = `SyntaxError: invalid response data`;
  } else {
    msg = `UnexpectedError: ${err}`;
  }

  console.error(`!!! ${msg}`);
  message.error(msg);
}
