import { message } from "antd";

const Settings = {
  initAt: null,
  publicUrl: "",
  headers: {},
  token: "",
  apiAddress: "",
};

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
    if (!d) {
      return;
    }

    Settings.apiAddress = d.apiAddress;
    console.log(`==> Got configs: ${JSON.stringify(d)}`);
  });
}

//
export function getApiAddress() {
   return Settings.ApiAddress;
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

  request(`${Settings.api}${path}`, options, callback);
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

  request(`${Settings.api}${path}`, options, callback);
}

export function request(path, options, callback=null) {
  if (Settings.token) {
    options.headers["Authorization"] = `Bearer ${Settings.token}`;
  }

  options.headers["X-TZ-Offset"] = Settings.initAt.getTimezoneOffset();

  fetch(`${path}`, options)
    .then(response => {
      // if (response.length === 0) {
      //   return null;
      // }
      let contentType = response.headers.get("Content-Type");
      if (!contentType || !contentType.startsWith("application/json")) {
        throw new TypeError("invalid response");
      }

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

      return response.json();
    }).then(res => {
      if (!res) {
        throw new TypeError("empty response");
      }


      if (res.code < 0) {
        message.warn(res.msg);
      } else if (res.code > 0) {
        message.error(res.msg);
      }
      if (res.code !== 0 && callback) {
        callback(null, res);
        return;
      }

      if (res.code === 0 && options.method === "GET" && res.data.hasOwnProperty("items")) {
        if (Array.isArray(res.data.items) && res.data.items.length === 0) {
          console.warn("!!! Have no items");
        }
      }

      if (callback) callback(res, null);
    })
    .catch(function (err) {
      console.error(`!!! http${options.method} ${path}: ${err}`);

      if (err instanceof TypeError && err.message.startsWith("NetworkError")) {
        message.error("NetworkError: request failed");
        return;
      } else if (err instanceof TypeError)  {
        message.error(`TypeError: ${err.message}`);
        return;
      } else if (err instanceof SyntaxError)  {
        message.error(`SyntaxError: invalid response data`);
        return;
      } else {
        message.error(`UnexpectedError: ${err}`);
      }
    });
}
