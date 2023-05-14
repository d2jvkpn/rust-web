// misc
export function helloWorld() {
  console.debug("~~~ Hello, world!");

  let num = window.prompt("Please enter a number:", "42");
  num = parseInt(num, 10);
  if (isNaN(num) || num <= 0) {
    num = 42;
  }

  console.log(`~~~ num: ${num}`);
}

export function randString(length) {
  let result   = [];
  let chars    = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let charsLen = chars.length;

  for ( let i = 0; i < length; i++ ) {
    result.push(chars.charAt(Math.floor(Math.random() * charsLen)));
  }

  return result.join('');
  // var crypto = require("crypto");
  // return crypto.randomBytes(Math.round(length/2)).toString('hex').slice(0, length);
}

export function fixStringLength(str, len=12) {
  return str.length > len ? str.slice(0, len-3) + "..." : str;
}

// time
export function datetime(at=null) {
  if (!at) {
    at = new Date();
  } else {
    at = new Date(at);
  }
  function padH0 (value, len=2) { return value.toString().padStart(len, '0')}

  function timezoneOffset(offset) {
    if (offset === 0) {
      return "Z";
    }

    let hour = padH0(Math.floor(Math.abs(offset) / 60));
    let minute = padH0(Math.abs(offset) % 60);
    return `${(offset < 0) ? "+" : "-"}${hour}:${minute}`;
  }

  at.date = `${at.getFullYear()}-${padH0(at.getMonth() + 1)}-${padH0(at.getDate())}`;
  at.time = `${padH0(at.getHours())}:${padH0(at.getMinutes())}:${padH0(at.getSeconds())}`;
  at.ms = padH0(at.getMilliseconds(), 3);
  at.tz = timezoneOffset(at.getTimezoneOffset());

  at.datetime = `${at.date}T${at.time}`;
  at.rfc3339 = at.datetime + `${at.tz}`;
  at.rfc3339ms = at.datetime + `.${at.ms}${at.tz}`;

  return at
}

export function datetimeAsFilename(at=null) {
  let now = datetime(at);
  return `${now.date}T${now.time.replaceAll(":", "-")}`;
}

export function localDatetime(str) {
  if (!str) {
    return "-";
  }

  let at = datetime(new Date(str));
  return at.date + " " + at.time;
}

// golang time.Time.AddDate(year, month, days int)
export function addDate(at, years, months, days) {
    if (!at) at = new Date();
    at.setDate(at.getDate() + days);
    at.setMonth(at.getMonth() + months);
    at.setFullYear(at.getFullYear() + years);
    at = datetime(at);
    return at;
}

//module.exports = {
//  helloWorld, randString, fixString,
//  datetime, localDatetime, addDate,
//}

export function removeUrlParams(url, keys) {
   if (!url) {
     url = window.location.href;
   }

   let href = new URL(url);
   let params = new URLSearchParams(href.search);
   keys.forEach(e => params.delete(e));
   href.search = params;

   return href;
}
