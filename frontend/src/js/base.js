import { message } from "antd";

const Settings = {
  initAt: null,
  publicUrl: "",
  headers: {},
  apiAddress: "",
};

const Interval = 15*1000; // 15s

export function load() {
  if (Settings.initAt !== null) {
    return;
  }

  Settings.publicUrl = process.env.PUBLIC_URL || "";
  Settings.initAt = new Date();

  let url = new URL(window.location.href);
  url = `${url.protocol}//${url.host}`;

  let p = process.env.PUBLIC_URL ? `${process.env.PUBLIC_URL}/configs.json` : "/configs.json";

  request(`${url}${p}`, {method: "GET", headers: {}}, function(d) {
    Settings.apiAddress = d.apiAddress;
    console.log(`==> Got configs: ${JSON.stringify(d)}`);

    setHeader("X-TZ-Offset", Settings.initAt.getTimezoneOffset());
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

  let delta = tokens.refreshExp*1000 - Math.round(new Date().getTime());

  if (delta < Interval) {
    return false;
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

      if (res.code && res.code !== 0) {
        message.warning(res.msg);
        console.log(`!!! response error: code=${res.code}, msg=${res.msg}`);
        return;
      }

      if (res.code === 0 && options.method === "GET" && res.data.hasOwnProperty("items")) {
        if (Array.isArray(res.data.items) && res.data.items.length === 0) {
          console.warning("!!! Have no items");
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
  if (err instanceof TypeError && err.message.startsWith("NetworkError")) {
    console.error("NetworkError: request failed");
  } else if (err instanceof TypeError)  {
    console.error(`TypeError: ${err.message}`);
  } else if (err instanceof SyntaxError)  {
    console.error(`SyntaxError: invalid response data`);
  } else {
    console.error(`UnexpectedError: ${err}`);
  }
}
